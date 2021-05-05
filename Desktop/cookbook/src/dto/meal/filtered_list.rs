use crate::models::meal::filter::AllMealFilteredApi;

#[derive(Debug)]
pub struct MealFilteredList {
    pub filtered_meals: Vec<Meal>,
}
impl MealFilteredList {
    pub fn default() -> Self {
        let filtered_meals = Vec::<Meal>::new();
        Self { filtered_meals }
    }

    pub fn from_api(meal_list: AllMealFilteredApi) -> Self {
        let mut filtered_meals = Vec::<Meal>::new();

        for m in meal_list.meals {
            let id_meal = match m.id_meal {
                Some(s) => s,
                None => String::from("0"),
            };
            let str_meal = match m.str_meal {
                Some(s) => s,
                None => String::default(),
            };
            let str_meal_thumb = match m.str_meal_thumb {
                Some(s) => s,
                None => String::default(),
            };
            let meal = Meal {
                id_meal,
                str_meal,
                str_meal_thumb,
            };
            filtered_meals.push(meal);
        }

        Self { filtered_meals }
    }
}

#[derive(Debug)]
pub struct Meal {
    pub str_meal: String,
    pub str_meal_thumb: String,
    pub id_meal: String,
}
