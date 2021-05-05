pub mod area;
pub mod categories;
pub mod filter;
pub mod ingredient;

use serde::Deserialize;
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllMealsApi {
    pub meals: Vec<MealApi>,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MealApi {
    pub id_meal: Option<String>,
    pub str_meal: Option<String>,
    pub str_drink_alternate: Option<String>,
    pub str_category: Option<String>,
    pub str_area: Option<String>,
    pub str_instructions: Option<String>,
    pub str_meal_thumb: Option<String>,
    pub str_tags: Option<String>,
    pub str_youtube: Option<String>,
    pub str_ingredient1: Option<String>,
    pub str_ingredient2: Option<String>,
    pub str_ingredient3: Option<String>,
    pub str_ingredient4: Option<String>,
    pub str_ingredient5: Option<String>,
    pub str_ingredient6: Option<String>,
    pub str_ingredient7: Option<String>,
    pub str_ingredient8: Option<String>,
    pub str_ingredient9: Option<String>,
    pub str_ingredient10: Option<String>,
    pub str_ingredient11: Option<String>,
    pub str_ingredient12: Option<String>,
    pub str_ingredient13: Option<String>,
    pub str_ingredient14: Option<String>,
    pub str_ingredient15: Option<String>,
    pub str_ingredient16: Option<String>,
    pub str_ingredient17: Option<String>,
    pub str_ingredient18: Option<String>,
    pub str_ingredient19: Option<String>,
    pub str_ingredient20: Option<String>,
    pub str_measure1: Option<String>,
    pub str_measure2: Option<String>,
    pub str_measure3: Option<String>,
    pub str_measure4: Option<String>,
    pub str_measure5: Option<String>,
    pub str_measure6: Option<String>,
    pub str_measure7: Option<String>,
    pub str_measure8: Option<String>,
    pub str_measure9: Option<String>,
    pub str_measure10: Option<String>,
    pub str_measure11: Option<String>,
    pub str_measure12: Option<String>,
    pub str_measure13: Option<String>,
    pub str_measure14: Option<String>,
    pub str_measure15: Option<String>,
    pub str_measure16: Option<String>,
    pub str_measure17: Option<String>,
    pub str_measure18: Option<String>,
    pub str_measure19: Option<String>,
    pub str_measure20: Option<String>,
    pub str_source: Option<String>,
    pub str_image_source: Option<String>,
    pub str_creative_commons_confirmed: Option<String>,
    pub date_modified: Option<String>,
}
