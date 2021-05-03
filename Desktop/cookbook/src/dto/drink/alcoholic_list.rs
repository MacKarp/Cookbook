use crate::models::drink::alcoholic::AllAlcoholicAPI;

#[derive(Debug)]
pub struct AlcoholicCategoryList {
    pub categories: Vec<String>,
}
impl AlcoholicCategoryList {
    pub fn default() -> Self {
        let categories = Vec::<String>::new();
        Self { categories }
    }

    pub fn from_api(alcoholic_categories: AllAlcoholicAPI) -> Self {
        let mut categories = Vec::<String>::new();

        for c in alcoholic_categories.drinks {
            match c.str_alcoholic {
                Some(category) => categories.push(category),
                None => break,
            }
        }

        Self { categories }
    }
}
