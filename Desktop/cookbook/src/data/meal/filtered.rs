use crate::dto::meal::filtered_list::MealFilteredList;
use crate::models::meal::filter::AllMealFilteredAPI;

use reqwest::blocking::get;

pub fn get_filtered_meal_category_items(value: &(String, Option<String>)) -> MealFilteredList {
    let selected = value.0.clone();
    let category = value.1.clone().unwrap();
    let link = match category.as_str() {
        "Categories" => {
            format!("https://www.themealdb.com/api/json/v1/1/filter.php?c=") + &selected
        }
        "Area" => format!("https://www.themealdb.com/api/json/v1/1/filter.php?a=") + &selected,

        "Ingredients" => {
            format!("https://www.themealdb.com/api/json/v1/1/filter.php?i=") + &selected
        }

        _ => format!("Default"),
    };
    get_filtered_meal_from_api(link)
}

fn get_filtered_meal_from_api(url: String) -> MealFilteredList {
    if url == "Default" {
        return MealFilteredList::default();
    }

    let filtered_list: AllMealFilteredAPI = get(url)
        .unwrap()
        .json()
        .unwrap_or(AllMealFilteredAPI::default());

    MealFilteredList::from_api(filtered_list)
}
