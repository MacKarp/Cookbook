use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllIngredientsApi {
    pub meals: Vec<IngredientApi>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngredientApi {
    pub id_ingredient: Option<String>,
    pub str_ingredient: Option<String>,
    pub str_description: Option<String>,
    pub str_type: Option<String>,
}
