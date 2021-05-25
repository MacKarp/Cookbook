use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use cookbook::dto::meal::recipe::MealRecipe;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FavoriteMeal {
    pub created_at: Option<DateTime<Utc>>,
    pub meal_area: Option<String>,
    pub meal_category: Option<String>,
    pub meal_id: Option<String>,
    pub meal_name: Option<String>,
    pub meal_photo_url: Option<String>,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
}
impl FavoriteMeal {
    pub fn from_meal_recipe(meal_recipe: MealRecipe, user_info: (&String, &String)) -> Self {
        let created_at = Some(chrono::Utc::now());
        let meal_area = Some(meal_recipe.area);
        let meal_category = Some(meal_recipe.category);
        let meal_id = Some(meal_recipe.id.to_string());
        let meal_name = Some(meal_recipe.meal_name);
        let meal_photo_url = Some(meal_recipe.thumb);
        let user_id = Some(user_info.0.clone());
        let user_name = Some(user_info.1.clone());

        Self {
            created_at,
            meal_area,
            meal_category,
            meal_id,
            meal_name,
            meal_photo_url,
            user_id,
            user_name,
        }
    }
}
