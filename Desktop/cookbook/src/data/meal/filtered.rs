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


#[test]
fn get_categories_filtered_meal_category_items() {
    let value = (
        String::from("Beef"),
        Some(String::from("Categories"))
    );

    // First drink name returned from API, it can change!
    let should_be=String::from("Beef and Mustard Pie"); 

    let tested = get_filtered_meal_category_items(&value).filtered_meals;
    let tested = tested[0].str_meal.clone();

    assert_eq!(
        should_be,tested,
        "Incorecct filtered 'Meal Categories' name, check if API is online and first drink name is: Beef and Mustard Pie"
    );
}

#[test]
fn get_area_filtered_meal_category_items() {
    let value = (
        String::from("American"),
        Some(String::from("Area"))
    );

    // First drink name returned from API, it can change!
    let should_be=String::from("Banana Pancakes"); 

    let tested = get_filtered_meal_category_items(&value).filtered_meals;
    let tested = tested[0].str_meal.clone();

    assert_eq!(
        should_be,tested,
        "Incorecct filtered 'Meal Area' name, check if API is online and first drink name is: Banana Pancakes"
    );
}

#[test]
fn get_ingredients_filtered_meal_category_items() {
    let value = (
        String::from("Chicken"),
        Some(String::from("Ingredients"))
    );

    // First drink name returned from API, it can change!
    let should_be=String::from("Brown Stew Chicken"); 

    let tested = get_filtered_meal_category_items(&value).filtered_meals;
    let tested = tested[0].str_meal.clone();

    assert_eq!(
        should_be,tested,
        "Incorecct filtered 'Meal Ingredients' name, check if API is online and first drink name is: Brown Stew Chicken"
    );
}