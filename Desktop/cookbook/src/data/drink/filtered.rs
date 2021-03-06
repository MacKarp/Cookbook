use crate::dto::drink::filtered_list::DrinkFilteredList;
use crate::models::drink::filter::AllDrinkFilteredApi;

use reqwest::blocking::get;

pub fn get_filtered_drink_category_items(value: &(String, Option<String>)) -> DrinkFilteredList {
    let selected = value.0.clone();
    let category = value.1.clone().unwrap();
    let link = match category.as_str() {
        "Alcoholic" => {
            "https://www.thecocktaildb.com/api/json/v1/1/filter.php?a=".to_string() + &selected
        }
        "Categories" => {
            "https://www.thecocktaildb.com/api/json/v1/1/filter.php?c=".to_string() + &selected
        }

        "Glass" => {
            "https://www.thecocktaildb.com/api/json/v1/1/filter.php?g=".to_string() + &selected
        }

        "Ingredients" => {
            "https://www.thecocktaildb.com/api/json/v1/1/filter.php?i=".to_string() + &selected
        }

        _ => "Default".to_string(),
    };
    get_filtered_drink_from_api(link)
}

fn get_filtered_drink_from_api(url: String) -> DrinkFilteredList {
    if url == "Default" {
        return DrinkFilteredList::default();
    }

    let filtered_list: AllDrinkFilteredApi = get(url)
        .unwrap()
        .json()
        .unwrap_or_else(|_| AllDrinkFilteredApi::default());

    DrinkFilteredList::from_api(filtered_list)
}

#[test]
fn get_alcoholic_filtered_drink_category_items() {
    let value = (String::from("Alcoholic"), Some(String::from("Alcoholic")));

    // First drink name returned from API, it can change!
    let should_be = String::from("1-900-FUK-MEUP");

    let tested = get_filtered_drink_category_items(&value).filtered_drinks;
    let tested = tested[0].str_drink.clone();

    assert_eq!(
        should_be,tested,
        "Incorecct filtered 'Drink Alcoholic' name, check if API is online and first drink name is: 1-900-FUK-MEUP"
    );
}

#[test]
fn get_categories_filtered_drink_category_items() {
    let value = (
        String::from("Ordinary Drink"),
        Some(String::from("Categories")),
    );

    // First drink name returned from API, it can change!
    let should_be = String::from("3-Mile Long Island Iced Tea");

    let tested = get_filtered_drink_category_items(&value).filtered_drinks;
    let tested = tested[0].str_drink.clone();

    assert_eq!(
        should_be,tested,
        "Incorecct filtered 'Drink Categories' name, check if API is online and first drink name is: 3-Mile Long Island Iced Tea"
    );
}

#[test]
fn get_glass_filtered_drink_category_items() {
    let value = (String::from("Highball glass"), Some(String::from("Glass")));

    // First drink name returned from API, it can change!
    let should_be = String::from("57 Chevy with a White License Plate");

    let tested = get_filtered_drink_category_items(&value).filtered_drinks;
    let tested = tested[0].str_drink.clone();

    assert_eq!(
        should_be,tested,
        "Incorecct filtered 'Drink Glass' name, check if API is online and first drink name is: 57 Chevy with a White License Plate"
    );
}

#[test]
fn get_ingredients_filtered_drink_category_items() {
    let value = (String::from("Light rum"), Some(String::from("Ingredients")));

    // First drink name returned from API, it can change!
    let should_be = String::from("151 Florida Bushwacker");

    let tested = get_filtered_drink_category_items(&value).filtered_drinks;
    let tested = tested[0].str_drink.clone();

    assert_eq!(
        should_be,tested,
        "Incorecct filtered 'Drink Ingredients' name, check if API is online and first drink name is: 151 Florida Bushwacker"
    );
}
