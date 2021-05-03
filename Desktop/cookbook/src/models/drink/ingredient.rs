use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllIngredientAPI {
    pub drinks: Vec<IngredientAPI>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngredientAPI {
    pub str_ingredient1: Option<String>,
}
