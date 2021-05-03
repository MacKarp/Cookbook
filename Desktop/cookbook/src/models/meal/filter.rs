use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllMealFilteredAPI {
    pub meals: Vec<MealAPI>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MealAPI {
    pub str_meal: Option<String>,
    pub str_meal_thumb: Option<String>,
    pub id_meal: Option<String>,
}
