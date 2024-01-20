extern crate alloc;

use icu::locid::Locale;
use icu_provider::{AsDeserializingBufferProvider, DataPayload, DataProvider, DataRequest};
use icu_provider_adapters::fallback::LocaleFallbackProvider;
use icu_provider_blob::BlobDataProvider;

pub enum MyError {
    MissingLocale,
}

fn main() {
    pub fn get_blob_provider(path: &str) -> LocaleFallbackProvider<BlobDataProvider> {
        let new_blob: Box<[u8]> = std::fs::read(path)
            .expect("blob should read successfully")
            .into();

        let provider = BlobDataProvider::try_new_from_blob(new_blob.into())
            .expect("deserialization should succeed");

        LocaleFallbackProvider::try_new_with_buffer_provider(provider)
            .expect("Provider should contain fallback rules")
    }

    pub fn get_date_skeleton_for_key(
        blob_path: &str,
        _locale_code: &str,
        key: &str,
    ) -> Result<String, MyError> {
        let _locale_code = _locale_code.to_owned() + "-u-ca-gregory";
        let name = _locale_code.as_bytes();
        let locale: Locale = Locale::try_from_bytes(name).unwrap();

        // ====LOOK HERE #2====
        // Use either the blob provider, and then deserialize it
        let provider = get_blob_provider(blob_path);
        let provider = provider.as_deserializing();
        // Or use the baked provider.
        // let provider = icu_datetime::provider::Baked;

        let req = DataRequest {
            locale: &locale.into(),
            metadata: Default::default(),
        };

        // // Grab all the skeletons for the locale
        let skeletons: DataPayload<icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker> =
            match provider.load(req) {
                Ok(payload) => payload.take_payload().expect("Failed to retrieve payload"),
                Err(_) => return Err(MyError::MissingLocale),
            };

        // println!("Skeletons: {:?}", skeletons.get());

        // Convert the key to a SkeletonV1
        let skeleton_key = icu_datetime::provider::calendar::SkeletonV1::try_from(key).unwrap();
        // Grab the pattern for the given key
        let pattern_match = match skeletons.get().0.get(&skeleton_key) {
            Some(pattern) => match pattern {
                icu_datetime::pattern::runtime::PatternPlurals::SinglePattern(pattern) => pattern,
                icu_datetime::pattern::runtime::PatternPlurals::MultipleVariants(_) => {
                    panic!("Found multiple patterns for skeleton: {:?}", skeleton_key)
                }
            },
            None => panic!("No pattern found for skeleton: {:?}", skeleton_key),
        };

        // println!("Pattern match: {:?}", pattern_match);

        let mut pattern_string = String::new();

        // Convert the pattern to a string
        for item in pattern_match.items.iter() {
            match item {
                icu_datetime::pattern::PatternItem::Field(field) => match field.symbol {
                    icu_datetime::fields::FieldSymbol::Day(_) => {
                        pattern_string.push_str("d");
                    }
                    icu_datetime::fields::FieldSymbol::Month(_) => {
                        pattern_string.push_str("M");
                    }
                    icu_datetime::fields::FieldSymbol::Year(_) => {
                        pattern_string.push_str("y");
                    }
                    _ => {}
                },
                icu_datetime::pattern::PatternItem::Literal(literal) => {
                    pattern_string.push_str(&literal.to_string());
                }
            }
        }

        // println!("Pattern string: {}", pattern_string);
        Ok(pattern_string)
    }
    pub fn test_all_locales(test_blob_path: &str) {
        println!("Testing blob {:?}", test_blob_path);
        let all_locales = [
            "ar_AE", "ar_BH", "ar_EG", "ar_IQ", "ar_LB", "ar_QA", "ar_SA", "ar_TN", "bg_BG",
            "cs_CZ", "da_DK", "de_AT", "de_BE", "de_CH", "de_DE", "de_LU", "el_GR", "en_AU",
            "en_CA", "en_GB", "en_HK", "en_IE", "en_IN", "en_NZ", "en_PH", "en_SG", "en_US",
            "en_ZA", "es_AR", "es_CL", "es_CO", "es_CR", "es_DO", "es_EC", "es_ES", "es_GT",
            "es_HN", "es_MX", "es_NI", "es_PA", "es_PE", "es_PR", "es_PY", "es_SV", "es_US",
            "es_UY", "es_VE", "et_EE", "fi_FI", "fr_BE", "fr_CA", "fr_CH", "fr_FR", "fr_LU",
            "he_IL", "hr_HR", "hu_HU", "id_ID", "is_IS", "it_CH", "it_IT", "ja_JP", "ko_KR",
            "lt_LT", "lv_LV", "ms_MY", "nl_BE", "nl_NL", "no_NO", "pl_PL", "pt_BR", "pt_PT",
            "ro_RO", "ru_RU", "sk_SK", "sl_SI", "sr_RS", "sv_SE", "th_TH", "tr_TR", "uk_UA",
            "vi_VN", "zh_CN", "zh_HK", "zh_TW",
        ];
        // let all_locales = [
        //     "ar", "bg", "cs", "da", "de", "el", "en", "es", "et", "fi", "fr", "he", "hr", "hu",
        //     "id", "is", "it", "ja", "ko", "lt", "lv", "ms", "nl", "no", "pl", "pt", "ro", "ru",
        //     "sk", "sl", "sr", "sv", "th", "tr", "uk", "vi", "zh",
        // ];

        let mut passed: Vec<&str> = Vec::new();
        let mut failed: Vec<&str> = Vec::new();

        for locale in all_locales {
            let val = get_date_skeleton_for_key(test_blob_path, locale, "dMy");

            match val {
                Ok(_) => passed.push(locale),
                Err(_) => failed.push(locale),
            }
        }

        println!("Passed: {:?}\nTotal {:?}/85\n", passed, passed.len());
        println!("Failed: {:?}\nTotal: {:?}/85\n", failed, failed.len());
        println!("=====================\n\n")
    }
    // SWITCH BETWEEN THESE TWO TO TEST THE DIFFERENT BLOBS
    const SPECIFIED_BLOB_PATH: &str = "specified.blob";
    const UN_SPECIFIED_BLOB_PATH: &str = "non-specified.blob";
    const INCLUDE_ZH_LOCALES: &str = "non-specified-zh.blob";

    test_all_locales(SPECIFIED_BLOB_PATH);
    test_all_locales(UN_SPECIFIED_BLOB_PATH);
    test_all_locales(INCLUDE_ZH_LOCALES);
}

// We have a list of locales that we support. However we are running into an issue with
// certain locales and the data_gen command.

// `icu_datagen` v1.4.1 is installed

// The first thing to do is run both of these commands below to generate the blobs to be used in the tests.
// #1 includes all specific locale + regions. Named: "specified.blob"
// #2 excludes regions, only using the language code. Named: "non-specified.blob"
// #3 copies #2 but includes the zh_HK and zh_TW. Named: "non-specified-zh.blob"

// icu4x-datagen --keys all --locales ar_AE ar_BH ar_EG ar_IQ ar_LB ar_QA ar_SA ar_TN bg_BG cs_CZ da_DK de_AT de_BE de_CH de_DE de_LU el_GR en_AU en_CA en_GB en_HK en_IE en_IN en_NZ en_PH en_SG en_US en_ZA es_AR es_CL es_CO es_CR es_DO es_EC es_ES es_GT es_HN es_MX es_NI es_PA es_PE es_PR es_PY es_SV es_US es_UY es_VE et_EE fi_FI fr_BE fr_CA fr_CH fr_FR fr_LU he_IL hr_HR hu_HU id_ID is_IS it_CH it_IT ja_JP ko_KR lt_LT lv_LV ms_MY nl_BE nl_NL no_NO pl_PL pt_BR pt_PT ro_RO ru_RU sk_SK sl_SI sr_RS sv_SE th_TH tr_TR uk_UA vi_VN zh_CN zh_HK zh_TW --format blob --out specified.blob
// icu4x-datagen --keys all --locales ar bg cs da de el en es et fi fr he hr hu id is it ja ko lt lv ms nl no pl pt ro ru sk sl sr sv th tr uk vi zh --format blob --out non-specified.blob
// icu4x-datagen --keys all --locales zh_HK zh_TW ar bg cs da de el en es et fi fr he hr hu id is it ja ko lt lv ms nl no pl pt ro ru sk sl sr sv th tr uk vi zh --format blob --out non-specified-zh.blob

// All locales we support
// ar_AE ar_BH ar_EG ar_IQ ar_LB ar_QA ar_SA ar_TN bg_BG cs_CZ da_DK de_AT de_BE de_CH de_DE de_LU el_GR en_AU en_CA en_GB en_HK en_IE en_IN en_NZ en_PH en_SG en_US en_ZA es_AR es_CL es_CO es_CR es_DO es_EC es_ES es_GT es_HN es_MX es_NI es_PA es_PE es_PR es_PY es_SV es_US es_UY es_VE et_EE fi_FI fr_BE fr_CA fr_CH fr_FR fr_LU he_IL hr_HR hu_HU id_ID is_IS it_CH it_IT ja_JP ko_KR lt_LT lv_LV ms_MY nl_BE nl_NL no_NO pl_PL pt_BR pt_PT ro_RO ru_RU sk_SK sl_SI sr_RS sv_SE th_TH tr_TR uk_UA vi_VN zh_CN zh_HK zh_TW
