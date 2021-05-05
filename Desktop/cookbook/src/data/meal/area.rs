use crate::dto::meal::area_list::AreaCategoryList;
use crate::models::meal::area::AllAreaApi;

use reqwest::blocking::get;

pub fn get_area_category_list_from_api() -> Option<AllAreaApi> {
    let recieved_area_categories: AllAreaApi =
        get("https://www.themealdb.com/api/json/v1/1/list.php?a=list")
            .unwrap()
            .json()
            .unwrap();
    if recieved_area_categories.meals.get(0).is_some() {
        return Some(recieved_area_categories);
    }
    None
}

pub fn get_area_category_list() -> AreaCategoryList {
    match get_area_category_list_from_api() {
        Some(list) => AreaCategoryList::from_api(list),
        None => AreaCategoryList::default(),
    }
}

#[test]
fn get_area_category_list_test() {
    let should_be_category_list = get_area_category_list_from_api();
    assert!(
        should_be_category_list.is_some(),
        "Meal category list is 'None', is API online?"
    );
}
