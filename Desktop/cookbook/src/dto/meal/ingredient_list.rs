use crate::models::meal::ingredient::AllIngredientsApi;

#[derive(Debug)]
pub struct IngredientCategoryList {
    pub categories: Vec<String>,
}
impl IngredientCategoryList {
    pub fn default() -> Self {
        let categories = Vec::<String>::new();
        Self { categories }
    }

    pub fn from_api(ingredient_categories: AllIngredientsApi) -> Self {
        let mut categories = Vec::<String>::new();

        for c in ingredient_categories.meals {
            match c.str_ingredient {
                Some(category) => categories.push(category),
                None => break,
            }
        }

        Self { categories }
    }
}
