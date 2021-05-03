use crate::dto::meal::categories_list::MealCategoryList;
use crate::models::meal::categories::AllMealCategoriesAPI;

use reqwest::blocking::get;

pub fn get_meal_category_list_from_api() -> Option<AllMealCategoriesAPI> {
    let recieved_meal_categories: AllMealCategoriesAPI =
        get("https://www.themealdb.com/api/json/v1/1/list.php?c=list")
            .unwrap()
            .json()
            .unwrap();
    if recieved_meal_categories.meals.get(0).is_some() {
        return Some(recieved_meal_categories);
    }
    None
}

pub fn get_meal_category_list() -> MealCategoryList {
    match get_meal_category_list_from_api() {
        Some(list) => return MealCategoryList::from_api(list),
        None => return MealCategoryList::default(),
    };
}

#[test]
fn get_meal_category_list_test() {
    let should_be_category_list = get_meal_category_list_from_api();
    assert!(
        should_be_category_list.is_some(),
        "Meal category list is 'None', is API online?"
    );
}
