use crate::dto::drink::ingredient_list::IngredientCategoryList;
use crate::models::drink::ingredient::AllIngredientApi;

use reqwest::blocking::get;

pub fn get_ingredient_category_list_from_api() -> Option<AllIngredientApi> {
    let recieved_ingredient_list: AllIngredientApi =
        get("https://www.thecocktaildb.com/api/json/v1/1/list.php?i=list")
            .unwrap()
            .json()
            .unwrap();
    if recieved_ingredient_list.drinks.get(0).is_some() {
        return Some(recieved_ingredient_list);
    }
    None
}

pub fn get_drink_ingredient_list() -> IngredientCategoryList {
    match get_ingredient_category_list_from_api() {
        Some(list) => IngredientCategoryList::from_api(list),
        None => IngredientCategoryList::default(),
    }
}

#[test]
fn get_ingredient_category_list_test() {
    let should_be_category_list = get_ingredient_category_list_from_api();
    assert!(
        should_be_category_list.is_some(),
        "Ingredient category list is 'None', is API online?"
    );
}
