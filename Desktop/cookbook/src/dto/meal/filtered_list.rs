#[derive(Debug)]
pub struct MealFilteredList {
    pub filtered_meals: Vec<Meal>,
}
impl MealFilteredList {
    pub fn default() -> Self {
        let filtered_meals = Vec::<Meal>::new();
        Self { filtered_meals }
    }

    pub fn from_api(meal_filtered_category: &String) -> Self {
        let mut filtered_meals = Vec::<Meal>::new();

        //let meal_filtered = Vec::<Mea

        //for m in meal_filtered.meals {
        //}

        Self { filtered_meals }
    }
}

#[derive(Debug)]
pub struct Meal {
    pub str_meal: String,
    pub str_meal_thumb: String,
    pub id_meal: String,
}
