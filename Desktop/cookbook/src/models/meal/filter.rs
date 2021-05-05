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

impl AllMealFilteredAPI {
    pub fn default() -> Self {
        let ingredient = MealAPI {
            id_meal: Some(String::from("0")),
            str_meal: None,
            str_meal_thumb: None,
        };
        Self {
            meals: vec![ingredient],
        }
    }
}
