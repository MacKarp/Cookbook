use crate::dto::drink::alcoholic_list::AlcoholicCategoryList;
use crate::models::drink::alcoholic::AllAlcoholicAPI;

use reqwest::blocking::get;

pub fn get_alcoholic_category_list_from_api() -> Option<AllAlcoholicAPI> {
    let recieved_alcoholic_list: AllAlcoholicAPI =
        get("https://www.thecocktaildb.com/api/json/v1/1/list.php?a=list")
            .unwrap()
            .json()
            .unwrap();
    if recieved_alcoholic_list.drinks.get(0).is_some() {
        return Some(recieved_alcoholic_list);
    }
    None
}

pub fn get_alcoholic_list() -> AlcoholicCategoryList {
    match get_alcoholic_category_list_from_api() {
        Some(list) => return AlcoholicCategoryList::from_api(list),
        None => return AlcoholicCategoryList::default(),
    };
}

#[test]
fn get_alcoholic_category_list_test() {
    let should_be_category_list = get_alcoholic_category_list_from_api();
    assert!(
        should_be_category_list.is_some(),
        "Alcoholic category list is 'None', is API online?"
    );
}
