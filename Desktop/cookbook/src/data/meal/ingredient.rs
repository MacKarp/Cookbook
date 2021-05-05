use crate::dto::meal::ingredient_list::IngredientCategoryList;
use crate::models::meal::ingredient::AllIngredientsApi;

use reqwest::blocking::get;

pub fn get_ingredient_category_list_from_api() -> Option<AllIngredientsApi> {
    let recieved_ingredient_list: AllIngredientsApi =
        get("https://www.themealdb.com/api/json/v1/1/list.php?i=list")
            .unwrap()
            .json()
            .unwrap();
    if recieved_ingredient_list.meals.get(0).is_some() {
        return Some(recieved_ingredient_list);
    }
    None
}

pub fn get_meal_ingredient_list() -> IngredientCategoryList {
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
