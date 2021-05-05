use crate::models::drink::ingredient::AllIngredientApi;

#[derive(Debug)]
pub struct IngredientCategoryList {
    pub categories: Vec<String>,
}
impl IngredientCategoryList {
    pub fn default() -> Self {
        let categories = Vec::<String>::new();
        Self { categories }
    }

    pub fn from_api(ingredient_categories: AllIngredientApi) -> Self {
        let mut categories = Vec::<String>::new();

        for c in ingredient_categories.drinks {
            match c.str_ingredient1 {
                Some(category) => categories.push(category),
                None => break,
            }
        }

        Self { categories }
    }
}
