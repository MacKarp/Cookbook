use crate::models::drink::categories::AllDrinkCategoryAPI;

#[derive(Debug)]
pub struct DrinkCategoryList {
    pub categories: Vec<String>,
}
impl DrinkCategoryList {
    pub fn default() -> Self {
        let categories = Vec::<String>::new();
        Self { categories }
    }

    pub fn from_api(drink_categories: AllDrinkCategoryAPI) -> Self {
        let mut categories = Vec::<String>::new();

        for c in drink_categories.drinks {
            match c.str_category {
                Some(category) => categories.push(category),
                None => break,
            }
        }

        Self { categories }
    }
}
