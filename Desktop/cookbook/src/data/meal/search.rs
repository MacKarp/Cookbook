use crate::dto::meal::recipe::MealRecipe;
use crate::models::meal::AllMealsApi;

use reqwest::blocking::get;

pub fn get_meal_recipe_by_search(searched: &str) -> Vec<MealRecipe> {
    let mut meals = Vec::<MealRecipe>::new();
    let searched_result = get_meal_by_search(searched);
    if searched_result.is_ok() {
        match searched_result.ok().unwrap() {
            Some(meal) => {
                for m in meal.meals {
                    meals.push(MealRecipe::from_meal(m));
                }
            }
            None => {
                meals.push(MealRecipe::default());
                return meals;
            }
        }
    }
    meals
}

fn get_meal_by_search(searched: &str) -> Result<Option<AllMealsApi>, reqwest::Error> {
    let url = "https://www.themealdb.com/api/json/v1/1/search.php?s=".to_string() + searched;

    let recieved_meals: AllMealsApi = get(url)?.json()?;
    if recieved_meals.meals.get(0).is_some() {
        return Ok(Some(recieved_meals));
    }
    Ok(None)
}

#[test]
fn get_meal_by_search_test() {
    let should_be_meal = get_meal_by_search(&String::from("Arrabiata")).unwrap();
    assert!(
        should_be_meal.is_some(),
        "Search meal by name is 'None', is API online?"
    );
}
