use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllIngredientApi {
    pub drinks: Vec<IngredientApi>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngredientApi {
    pub str_ingredient1: Option<String>,
}
