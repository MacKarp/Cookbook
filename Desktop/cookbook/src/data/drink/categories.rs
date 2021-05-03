use crate::dto::drink::categories_list::DrinkCategoryList;
use crate::models::drink::categories::AllDrinkCategoryAPI;

use reqwest::blocking::get;

pub fn get_drink_category_list_from_api() -> Option<AllDrinkCategoryAPI> {
    let recieved_drink_categories: AllDrinkCategoryAPI =
        get("https://www.thecocktaildb.com/api/json/v1/1/list.php?c=list")
            .unwrap()
            .json()
            .unwrap();
    if recieved_drink_categories.drinks.get(0).is_some() {
        return Some(recieved_drink_categories);
    }
    None
}

pub fn get_drink_category_list() -> DrinkCategoryList {
    match get_drink_category_list_from_api() {
        Some(list) => return DrinkCategoryList::from_api(list),
        None => return DrinkCategoryList::default(),
    };
}

#[test]
fn get_drink_category_list_test() {
    let should_be_category_list = get_drink_category_list_from_api();
    assert!(
        should_be_category_list.is_some(),
        "Drink category list is 'None', is API online?"
    );
}
