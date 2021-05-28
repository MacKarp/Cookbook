use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use cookbook::dto::drink::recipe::DrinkRecipe;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FavoriteDrink {
    pub created_at: Option<DateTime<Utc>>,
    pub drink_category: Option<String>,
    pub drink_id: Option<String>,
    pub drink_name: Option<String>,
    pub drink_photo_url: Option<String>,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
}
impl FavoriteDrink {
    pub fn from_drink_recipe(drink_recipe: DrinkRecipe, user_info: (&String, &String)) -> Self {
        let created_at = Some(chrono::Utc::now());
        let drink_category = Some(drink_recipe.category);
        let drink_id = Some(drink_recipe.id.to_string());
        let drink_name = Some(drink_recipe.drink_name);
        let drink_photo_url = Some(drink_recipe.thumb);
        let user_id = Some(user_info.0.clone());
        let user_name = Some(user_info.1.clone());

        Self {
            created_at,
            drink_category,
            drink_id,
            drink_name,
            drink_photo_url,
            user_id,
            user_name,
        }
    }
}
