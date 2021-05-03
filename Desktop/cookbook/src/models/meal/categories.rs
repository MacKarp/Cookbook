use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllMealCategoriesAPI {
    pub meals: Vec<MealAPI>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MealAPI {
    pub str_category: Option<String>,
}
