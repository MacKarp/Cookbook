use crate::dto::drink::filtered_list::DrinkFilteredList;
use crate::models::drink::filter::AllDrinkFilteredAPI;

use reqwest::blocking::get;

pub fn get_filtered_drink_category_items(value: &(String, Option<String>)) -> DrinkFilteredList {
    let selected = value.0.clone();
    let category = value.1.clone().unwrap();
    println!("test: {:?}", value);
    let link = match category.as_str() {
        "Alcoholic" => {
            format!("https://www.thecocktaildb.com/api/json/v1/1/filter.php?a=") + &selected
        }
        "Categories" => {
            format!("https://www.thecocktaildb.com/api/json/v1/1/filter.php?c=") + &selected
        }

        "Glass" => format!("https://www.thecocktaildb.com/api/json/v1/1/filter.php?g=") + &selected,

        "Ingredients" => {
            format!("https://www.thecocktaildb.com/api/json/v1/1/filter.php?i=") + &selected
        }

        _ => format!("Default"),
    };
    get_filtered_drink_from_api(link)
}

fn get_filtered_drink_from_api(url: String) -> DrinkFilteredList {
    if url == "Default" {
        return DrinkFilteredList::default();
    }

    let filtered_list: AllDrinkFilteredAPI = get(url)
        .unwrap()
        .json()
        .unwrap_or(AllDrinkFilteredAPI::default());

    DrinkFilteredList::from_api(filtered_list)
}
