use crate::models::meal::categories::AllMealCategoriesApi;

#[derive(Debug)]
pub struct MealCategoryList {
    pub categories: Vec<String>,
}
impl MealCategoryList {
    pub fn default() -> Self {
        let categories = Vec::<String>::new();
        Self { categories }
    }

    pub fn from_api(meal_categories: AllMealCategoriesApi) -> Self {
        let mut categories = Vec::<String>::new();

        for c in meal_categories.meals {
            match c.str_category {
                Some(category) => categories.push(category),
                None => break,
            }
        }

        Self { categories }
    }
}
