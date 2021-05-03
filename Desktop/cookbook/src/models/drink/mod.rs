pub mod alcoholic;
pub mod categories;
pub mod glass;
pub mod ingredient;

use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllDrinksAPI {
    pub drinks: Vec<DrinkAPI>,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DrinkAPI {
    pub id_drink: Option<String>,
    pub str_drink: Option<String>,
    pub str_drink_alternate: Option<String>,
    pub str_tags: Option<String>,
    pub str_video: Option<String>,
    pub str_category: Option<String>,
    #[serde(rename = "strIBA")]
    pub str_iba: Option<String>,
    pub str_alcoholic: Option<String>,
    pub str_glass: Option<String>,
    pub str_instructions: Option<String>,
    #[serde(rename = "strInstructionsES")]
    pub str_instructions_es: Option<String>,
    #[serde(rename = "strInstructionsDE")]
    pub str_instructions_de: Option<String>,
    #[serde(rename = "strInstructionsFR")]
    pub str_instructions_fr: Option<String>,
    #[serde(rename = "strInstructionsIT")]
    pub str_instructions_it: Option<String>,
    #[serde(rename = "strInstructionsZH-HANS")]
    pub str_instructions_zh_hans: Option<String>,
    #[serde(rename = "strInstructionsZH-HANT")]
    pub str_instructions_zh_hant: Option<String>,
    pub str_drink_thumb: Option<String>,
    pub str_ingredient1: Option<String>,
    pub str_ingredient2: Option<String>,
    pub str_ingredient3: Option<String>,
    pub str_ingredient4: Option<String>,
    pub str_ingredient5: Option<String>,
    pub str_ingredient6: Option<String>,
    pub str_ingredient7: Option<String>,
    pub str_ingredient8: Option<String>,
    pub str_ingredient9: Option<String>,
    pub str_ingredient10: Option<String>,
    pub str_ingredient11: Option<String>,
    pub str_ingredient12: Option<String>,
    pub str_ingredient13: Option<String>,
    pub str_ingredient14: Option<String>,
    pub str_ingredient15: Option<String>,
    pub str_measure1: Option<String>,
    pub str_measure2: Option<String>,
    pub str_measure3: Option<String>,
    pub str_measure4: Option<String>,
    pub str_measure5: Option<String>,
    pub str_measure6: Option<String>,
    pub str_measure7: Option<String>,
    pub str_measure8: Option<String>,
    pub str_measure9: Option<String>,
    pub str_measure10: Option<String>,
    pub str_measure11: Option<String>,
    pub str_measure12: Option<String>,
    pub str_measure13: Option<String>,
    pub str_measure14: Option<String>,
    pub str_measure15: Option<String>,
    pub str_image_source: Option<String>,
    pub str_image_attribution: Option<String>,
    pub str_creative_commons_confirmed: Option<String>,
    pub date_modified: Option<String>,
}
