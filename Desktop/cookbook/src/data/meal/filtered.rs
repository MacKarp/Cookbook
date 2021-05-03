use crate::dto::meal::filtered_list::MealFilteredList;

pub fn get_filtered_meal_category_items(category: &String) -> MealFilteredList {
    if category.len() > 0 {
        return get_filtered_meal_from_api(category);
    }
    MealFilteredList::default()
}

fn get_filtered_meal_from_api(meal_filtered_category: &String) -> MealFilteredList {
    let link = "http://www.themealdb.com/api/json/v1/1/filter.php?c=Seafood";
    MealFilteredList::default()
}
