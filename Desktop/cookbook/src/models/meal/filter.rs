use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllMealFilteredApi {
    pub meals: Vec<MealApi>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MealApi {
    pub str_meal: Option<String>,
    pub str_meal_thumb: Option<String>,
    pub id_meal: Option<String>,
}

impl AllMealFilteredApi {
    pub fn default() -> Self {
        let ingredient = MealApi {
            id_meal: Some(String::from("0")),
            str_meal: None,
            str_meal_thumb: None,
        };
        Self {
            meals: vec![ingredient],
        }
    }
}
