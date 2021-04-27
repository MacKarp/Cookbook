use crate::models::meal::api::area::AllAreaAPI;

#[derive(Debug)]
pub struct AreaCategoryList {
    pub categories: Vec<String>,
}
impl AreaCategoryList {
    pub fn default() -> Self {
        let categories = Vec::<String>::new();
        Self { categories }
    }

    pub fn from_api(area_categories: AllAreaAPI) -> Self {
        let mut categories = Vec::<String>::new();

        for c in area_categories.meals {
            match c.str_area {
                Some(category) => categories.push(category),
                None => break,
            }
        }

        Self { categories }
    }
}
