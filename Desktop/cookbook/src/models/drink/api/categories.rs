use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllDrinkCategoryAPI {
    pub drinks: Vec<DrinkCategoryAPI>,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DrinkCategoryAPI {
    pub str_category: Option<String>,
}
