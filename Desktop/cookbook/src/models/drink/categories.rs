use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllDrinkCategoryApi {
    pub drinks: Vec<DrinkCategoryApi>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrinkCategoryApi {
    pub str_category: Option<String>,
}
