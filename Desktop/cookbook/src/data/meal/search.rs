use crate::dto::meal::recipe::MealRecipe;
use crate::models::meal::AllMealsApi;

use reqwest::blocking::get;

pub fn get_meal_recipe_by_search(searched: &str) -> Vec<MealRecipe> {
    let mut meals = Vec::<MealRecipe>::new();
    match get_meal_by_search(searched) {
        Some(meal) => {
            for m in meal.meals {
                meals.push(MealRecipe::from_meal(m));
            }
        }
        None => meals.push(MealRecipe::default()),
    }
    meals
}

fn get_meal_by_search(searched: &str) -> Option<AllMealsApi> {
    let url = "https://www.themealdb.com/api/json/v1/1/search.php?s=".to_string() + searched;

    let recieved_meals: AllMealsApi = get(url).unwrap().json().unwrap();
    if recieved_meals.meals.get(0).is_some() {
        return Some(recieved_meals);
    }
    None
}

#[test]
fn get_meal_by_search_test() {
    let should_be_meal = get_meal_by_search(&String::from("Arrabiata"));
    assert!(
        should_be_meal.is_some(),
        "Search meal by name is 'None', is API online?"
    );
}
