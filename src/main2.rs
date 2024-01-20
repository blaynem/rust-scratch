use core::panic;

// use icu::datetime::pattern::PatternItem;
use icu::locid::Locale;
// use icu_datetime::pattern::runtime::PatternPlurals;
use icu_datetime::provider::calendar::{
    DateSkeletonPatternsV1Marker, GregorianDateLengthsV1Marker,
};
use icu_provider::{DataError, DataLocale, DataPayload, DataProvider, DataRequest, DataResponse};

pub struct ICU4XLocale(pub Locale);

impl ICU4XLocale {
    pub fn get_time_skeleton_for_key(&self, key: &str) -> String {
        // Convert the key to a SkeletonV1
        let req = DataRequest {
            locale: &DataLocale::from(&self.0),
            metadata: Default::default(),
        };

        // Grab all the skeletons for the locale
        let skeletonMarker: DataPayload<
            icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker,
        > = icu_datetime::provider::Baked
            .load(req)
            .expect("Failed to load payload")
            .take_payload()
            .expect("Failed to retrieve payload");

        let skeletons = skeletonMarker.get();

        // Grab the pattern for the given key
        let skeleton_key = icu_datetime::provider::calendar::SkeletonV1::try_from(key).unwrap();
        let pattern_match = match skeletons.0.get(&skeleton_key) {
            Some(pattern) => match pattern {
                icu_datetime::pattern::runtime::PatternPlurals::SinglePattern(pattern) => pattern,
                icu_datetime::pattern::runtime::PatternPlurals::MultipleVariants(_) => {
                    panic!("Found multiple patterns for skeleton: {:?}", skeleton_key)
                }
            },
            None => panic!("No pattern found for skeleton: {:?}", skeleton_key),
        };

        let mut pattern_string = String::new();

        // Convert the pattern to a string
        for item in pattern_match.items.iter() {
            match item {
                icu_datetime::pattern::PatternItem::Field(field) => match field.symbol {
                    icu_datetime::fields::FieldSymbol::Day(day) => {
                        println!("---day---{:?}", day);
                        pattern_string.push_str("d");
                    }
                    icu_datetime::fields::FieldSymbol::Month(month) => {
                        println!("---month---{:?}", month);
                        pattern_string.push_str("M");
                    }
                    icu_datetime::fields::FieldSymbol::Year(year) => {
                        println!("---year---{:?}", year);
                        pattern_string.push_str("y");
                    }
                    _ => {}
                },
                icu_datetime::pattern::PatternItem::Literal(literal) => {
                    println!("---literal---{:?}", literal);
                    pattern_string.push_str(&literal.to_string());
                }
            }
        }

        pattern_string
    }
    pub fn get_data_payload(
        &self,
    ) -> (
        DataPayload<GregorianDateLengthsV1Marker>,
        DataPayload<DateSkeletonPatternsV1Marker>,
    ) {
        let req = DataRequest {
            locale: &DataLocale::from(&self.0),
            metadata: Default::default(),
        };
        let patterns = icu::datetime::provider::Baked
            .load(req)
            .expect("Failed to load payload")
            .take_payload()
            .expect("Failed to retrieve payload");
        let skeletons: DataPayload<DateSkeletonPatternsV1Marker> = icu::datetime::provider::Baked
            .load(req)
            .expect("Failed to load payload")
            .take_payload()
            .expect("Failed to retrieve payload");
        (patterns, skeletons)
    }

    pub fn get_decimal_grouping_separator(&self) -> Result<(), ()> {
        let req = DataRequest {
            locale: &DataLocale::from(&self.0),
            metadata: Default::default(),
        };

        let test: DataPayload<GregorianDateLengthsV1Marker> = icu::datetime::provider::Baked
            .load(req)
            .unwrap()
            .take_payload()
            .unwrap();
        let thing: Result<DataResponse<DateSkeletonPatternsV1Marker>, DataError> =
            icu::datetime::provider::Baked.load(req);
        println!("{:?}", thing);

        Ok(())
    }

    /// Gets the Date Skeleton pattern string for the given key.
    /// Typically used for getting month+year ("mY") and year ("y") skeletons
    ///
    /// Note: Strings are from availableFormats in CLDR.
    /// See: https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-dates-full/main/af-NA/ca-gregorian.json#L333
    pub fn get_date_skeleton_for_key(&self, key: &str) -> String {
        // Convert the key to a SkeletonV1
        let skeleton_key = icu_datetime::provider::calendar::SkeletonV1::try_from(key).unwrap();
        let req = DataRequest {
            locale: &DataLocale::from(&self.0),
            metadata: Default::default(),
        };

        // Grab all the skeletons for the locale
        let skeletons: DataPayload<icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker> =
            icu_datetime::provider::Baked
                .load(req)
                .expect("Failed to load payload")
                .take_payload()
                .expect("Failed to retrieve payload");

        println!("---skeletons---{:?}", skeletons.get().0);
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

        let mut pattern_string = String::new();

        // Convert the pattern to a string
        for item in pattern_match.items.iter() {
            match item {
                icu_datetime::pattern::PatternItem::Field(field) => match field.symbol {
                    icu_datetime::fields::FieldSymbol::Day(day) => {
                        println!("---day---{:?}", day);
                        pattern_string.push_str("d");
                    }
                    icu_datetime::fields::FieldSymbol::Month(month) => {
                        println!("---month---{:?}", month);
                        pattern_string.push_str("M");
                    }
                    icu_datetime::fields::FieldSymbol::Year(year) => {
                        println!("---year---{:?}", year);
                        pattern_string.push_str("y");
                    }
                    _ => {}
                },
                icu_datetime::pattern::PatternItem::Literal(literal) => {
                    println!("---literal---{:?}", literal);
                    pattern_string.push_str(&literal.to_string());
                }
            }
        }

        pattern_string
    }
}

fn main() {
    let test_locale: Locale = "fr-u-ca-gregory".parse::<Locale>().unwrap().into();
    let _ = ICU4XLocale(test_locale);
    print!("Running...");

    // const YEAR_KEY: &str = "y";
    // const MONTH_YEAR_KEY: &str = "yMd";
    // const DAY_MONTH_YEAR_KEY: &str = "EBhm";

    // // let year_pattern = locale.get_date_skeleton_for_key(YEAR_KEY);
    // // let month_year_pattern = locale.get_date_skeleton_for_key(MONTH_YEAR_KEY);
    // let day_month_year_pattern = locale.get_time_skeleton_for_key(DAY_MONTH_YEAR_KEY);

    // // println!("year--{:?}", year_pattern);
    // // println!("month--{:?}", month_year_pattern);
    // println!("day--{:?}", day_month_year_pattern);
}
