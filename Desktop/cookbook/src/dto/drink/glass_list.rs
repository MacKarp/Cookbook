use crate::models::drink::glass::AllGlassApi;

#[derive(Debug)]
pub struct GlassCategoryList {
    pub categories: Vec<String>,
}
impl GlassCategoryList {
    pub fn default() -> Self {
        let categories = Vec::<String>::new();
        Self { categories }
    }

    pub fn from_api(glass_categories: AllGlassApi) -> Self {
        let mut categories = Vec::<String>::new();

        for c in glass_categories.drinks {
            match c.str_glass {
                Some(category) => categories.push(category),
                None => break,
            }
        }

        Self { categories }
    }
}
