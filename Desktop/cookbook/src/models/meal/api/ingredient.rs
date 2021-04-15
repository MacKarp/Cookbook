use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllIngredientsAPI {
    pub meals: Vec<Meal>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngredientAPI {
    pub id_ingredient: Option<String>,
    pub str_ingredient: Option<String>,
    pub str_description: Option<String>,
    pub str_type: Option<String>,
}
