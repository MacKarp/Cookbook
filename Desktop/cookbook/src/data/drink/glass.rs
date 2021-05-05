use crate::dto::drink::glass_list::GlassCategoryList;
use crate::models::drink::glass::AllGlassApi;

use reqwest::blocking::get;

pub fn get_glass_category_list_from_api() -> Option<AllGlassApi> {
    let recieved_glass_list: AllGlassApi =
        get("https://www.thecocktaildb.com/api/json/v1/1/list.php?g=list")
            .unwrap()
            .json()
            .unwrap();
    if recieved_glass_list.drinks.get(0).is_some() {
        return Some(recieved_glass_list);
    }
    None
}

pub fn get_glass_list() -> GlassCategoryList {
    match get_glass_category_list_from_api() {
        Some(list) => GlassCategoryList::from_api(list),
        None => GlassCategoryList::default(),
    }
}

#[test]
fn get_glass_category_list_test() {
    let should_be_category_list = get_glass_category_list_from_api();
    assert!(
        should_be_category_list.is_some(),
        "Glass category list is 'None', is API online?"
    );
}
