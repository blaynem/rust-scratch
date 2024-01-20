## Solution

Fixed in this (PR)[https://github.com/unicode-org/icu4x/pull/4533]!

## Whats wrong?

We have a list of locales that we support. However we are running into an issue with
certain locales and the data_gen command.

## How to replicate

Install icu_datagen, i'm using v1.4.1

```sh
cargo install icu_datagen
```

Next is to run the generate blob commands.
#1 includes all specific locale + regions. Named: "specified.blob"
#2 excludes regions, only using the language code. Named: "non-specified.blob"
#3 copies #2 but includes the zh_HK and zh_TW. Named: "non-specified-zh.blob"

```sh
icu4x-datagen --keys all --locales ar_AE ar_BH ar_EG ar_IQ ar_LB ar_QA ar_SA ar_TN bg_BG cs_CZ da_DK de_AT de_BE de_CH de_DE de_LU el_GR en_AU en_CA en_GB en_HK en_IE en_IN en_NZ en_PH en_SG en_US en_ZA es_AR es_CL es_CO es_CR es_DO es_EC es_ES es_GT es_HN es_MX es_NI es_PA es_PE es_PR es_PY es_SV es_US es_UY es_VE et_EE fi_FI fr_BE fr_CA fr_CH fr_FR fr_LU he_IL hr_HR hu_HU id_ID is_IS it_CH it_IT ja_JP ko_KR lt_LT lv_LV ms_MY nl_BE nl_NL no_NO pl_PL pt_BR pt_PT ro_RO ru_RU sk_SK sl_SI sr_RS sv_SE th_TH tr_TR uk_UA vi_VN zh_CN zh_HK zh_TW --format blob --out specified.blob

icu4x-datagen --keys all --locales ar bg cs da de el en es et fi fr he hr hu id is it ja ko lt lv ms nl no pl pt ro ru sk sl sr sv th tr uk vi zh --format blob --out non-specified.blob

icu4x-datagen --keys all --locales zh_HK zh_TW ar bg cs da de el en es et fi fr he hr hu id is it ja ko lt lv ms nl no pl pt ro ru sk sl sr sv th tr uk vi zh --format blob --out non-specified-zh.blob
```

After installation you can do `cargo run` and it will print out a log like below. Please also test with the `Baked` provider by going [here](src/main.rs#39) and uncommenting that line to do another test to see how it differs.

Uncomment this line:

```rust
let provider = icu_datetime::provider::Baked;
```

Example log:

```
Testing blob "non-specified-zh.blob"
Passed: ["ar_AE", "ar_BH", "ar_EG", "ar_IQ", ...]
Total 84/85

Failed: ["zh_TW"]
Total: 1/85
```

### Whoopdy doo, what does it all mean?

Well, the first thing to notice is that if we use the baked data, all of these locales pass the test. If you then use the generated data, you'll notice that each test differs a bit.

#### "specified.blob"

Half of the locales fail, despite us including the locale+region specifically.

#### "non-specified.blob"

Most of the locales pass when we exclude the region code, still 2 are failing `zh_HK` and `zh_TW`.

#### "non-specified-zh.blob"

We now specifically incldue the 2 failing codes, yet `zh_TW` still fails.

#### Assumption

My assumption was the generated data from `icu-datagen` would be a drop in replacement of the baked data.

### Well how do we know the blob data isn't wrong?

Uh, good question. Kinda hard to tell. But! We can actually call the same `icu4x-datagen` command but this time with the format set to `dir` and we can see the data compiled to json. It seems like the correct data is generated. The below command is the exact same as the first one we used to generate blobs. The only differences are we changed from `--format blob --out specified.blob` -> `--format dir --out specified`

```sh
icu4x-datagen --keys all --locales ar_AE ar_BH ar_EG ar_IQ ar_LB ar_QA ar_SA ar_TN bg_BG cs_CZ da_DK de_AT de_BE de_CH de_DE de_LU el_GR en_AU en_CA en_GB en_HK en_IE en_IN en_NZ en_PH en_SG en_US en_ZA es_AR es_CL es_CO es_CR es_DO es_EC es_ES es_GT es_HN es_MX es_NI es_PA es_PE es_PR es_PY es_SV es_US es_UY es_VE et_EE fi_FI fr_BE fr_CA fr_CH fr_FR fr_LU he_IL hr_HR hu_HU id_ID is_IS it_CH it_IT ja_JP ko_KR lt_LT lv_LV ms_MY nl_BE nl_NL no_NO pl_PL pt_BR pt_PT ro_RO ru_RU sk_SK sl_SI sr_RS sv_SE th_TH tr_TR uk_UA vi_VN zh_CN zh_HK zh_TW --format dir --out specified
```

### All locales we support

```
ar_AE ar_BH ar_EG ar_IQ ar_LB ar_QA ar_SA ar_TN bg_BG cs_CZ da_DK de_AT de_BE de_CH de_DE de_LU el_GR en_AU en_CA en_GB en_HK en_IE en_IN en_NZ en_PH en_SG en_US en_ZA es_AR es_CL es_CO es_CR es_DO es_EC es_ES es_GT es_HN es_MX es_NI es_PA es_PE es_PR es_PY es_SV es_US es_UY es_VE et_EE fi_FI fr_BE fr_CA fr_CH fr_FR fr_LU he_IL hr_HR hu_HU id_ID is_IS it_CH it_IT ja_JP ko_KR lt_LT lv_LV ms_MY nl_BE nl_NL no_NO pl_PL pt_BR pt_PT ro_RO ru_RU sk_SK sl_SI sr_RS sv_SE th_TH tr_TR uk_UA vi_VN zh_CN zh_HK zh_TW
```
