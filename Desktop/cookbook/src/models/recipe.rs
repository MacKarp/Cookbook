use crate::models::meal::Meal;

#[derive(Debug)]
pub struct MealRecipe {
    pub id: u64,
    pub meal_name: String,
    pub drink_alternate: String,
    pub category: String,
    pub area: String,
    pub instructions: String,
    pub thumb: String,
    pub tags: String,
    pub youtube: String,
    pub ingredients: Vec<String>,
    pub source: String,
    pub image_source: String,
    pub creative_commons_confirmed: String,
    pub date_modified: String,
}
impl MealRecipe {
    pub fn default() -> Self {
        Self {
            id: 0,
            meal_name: "Default meal name".to_string(),
            drink_alternate: "Default drink alternate".to_string(),
            category: "Default category".to_string(),
            area: "Default area".to_string(),
            instructions: "instructions".to_string(),
            thumb: "Default thumb".to_string(),
            tags: "Default tags".to_string(),
            youtube: "Default youtube".to_string(),
            ingredients: Vec::<String>::new(),
            source: "Default source".to_string(),
            image_source: "Default image_source".to_string(),
            creative_commons_confirmed: "Default creative_commons_confirmed".to_string(),
            date_modified: "Default date modified".to_string(),
        }
    }
    pub fn from_meal(meal: Meal) -> Self {
        let id = match &meal.id_meal {
            Some(str_id) => str_id.clone().parse().unwrap_or_default(),
            None => 0,
        };
        let meal_name = match &meal.str_meal {
            Some(str) => str.clone(),
            None => "Default recipe name".to_string(),
        };

        let drink_alternate = match &meal.str_drink_alternate {
            Some(str) => str.clone(),
            None => "Default drink alternate".to_string(),
        };

        let category = match &meal.str_category {
            Some(str) => str.clone(),
            None => "Default category".to_string(),
        };

        let area = match &meal.str_area {
            Some(str) => str.clone(),
            None => "Default area".to_string(),
        };

        let instructions = match &meal.str_instructions {
            Some(str) => str.clone(),
            None => "Default instructions".to_string(),
        };

        let thumb = match &meal.str_meal_thumb {
            Some(str) => str.clone(),
            None => "Default thumb".to_string(),
        };
        let tags = match &meal.str_tags {
            Some(str) => str.clone(),
            None => "Default tags".to_string(),
        };

        let youtube = match &meal.str_youtube {
            Some(str) => str.clone(),
            None => "Default youtube link".to_string(),
        };

        let source = match &meal.str_source {
            Some(str) => str.clone(),
            None => "Default source".to_string(),
        };

        let image_source = match &meal.str_image_source {
            Some(str) => str.clone(),
            None => "Default image source".to_string(),
        };

        let creative_commons_confirmed = match &meal.str_creative_commons_confirmed {
            Some(str) => str.clone(),
            None => "Default creative common".to_string(),
        };

        let date_modified = match &meal.date_modified {
            Some(str) => str.clone(),
            None => "Default date".to_string(),
        };
        let ingredients = ingredients_list(&meal);
        Self {
            id,
            meal_name,
            drink_alternate,
            category,
            area,
            instructions,
            thumb,
            tags,
            youtube,
            ingredients,
            source,
            image_source,
            creative_commons_confirmed,
            date_modified,
        }
    }
}

fn ingredients_list(meal: &Meal) -> Vec<String> {
    let mut ingredients = Vec::new();

    let mut measured_ingredient = measure_ingredient(&meal.str_measure1, &meal.str_ingredient1);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&meal.str_measure2, &meal.str_ingredient2);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&meal.str_measure3, &meal.str_ingredient3);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&meal.str_measure4, &meal.str_ingredient4);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&meal.str_measure5, &meal.str_ingredient5);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&meal.str_measure6, &meal.str_ingredient6);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&meal.str_measure7, &meal.str_ingredient7);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&meal.str_measure8, &meal.str_ingredient8);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&meal.str_measure9, &meal.str_ingredient9);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&meal.str_measure10, &meal.str_ingredient10);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&meal.str_measure11, &meal.str_ingredient11);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&meal.str_measure12, &meal.str_ingredient12);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&meal.str_measure13, &meal.str_ingredient13);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&meal.str_measure14, &meal.str_ingredient14);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&meal.str_measure15, &meal.str_ingredient15);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&meal.str_measure16, &meal.str_ingredient16);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&meal.str_measure17, &meal.str_ingredient17);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&meal.str_measure18, &meal.str_ingredient18);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&meal.str_measure19, &meal.str_ingredient19);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&meal.str_measure20, &meal.str_ingredient20);
    ingredients.push(measured_ingredient);

    ingredients
}

fn measure_ingredient(measure: &Option<String>, ingredient: &Option<String>) -> String {
    let measure = match measure {
        Some(str) => str.clone().trim().to_string(),
        None => "".to_string(),
    };
    let ingredient = match ingredient {
        Some(str) => str.clone().trim().to_string(),
        None => "".to_string(),
    };
    let measured_ingredient = measure + " " + ingredient.as_str().trim();
    measured_ingredient
}

#[test]
fn ingredients_list_test() {
    let meal = crate::data::meal::get_random_meal().expect("Should be valid Meal");
    let should_be = ingredients_list(&meal);
    assert!(should_be.len() > 0);
}

#[test]
fn measure_ingredient_test() {
    let measure = &Some("1 tsp".to_string());
    let ingredient = &Some("Ginger".to_string());

    let tested = measure_ingredient(measure, ingredient);
    let should_be = String::from("1 tsp Ginger");

    assert_eq!(tested, should_be);
}
