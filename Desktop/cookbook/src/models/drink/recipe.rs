use super::api::DrinkAPI;
#[derive(Debug)]
pub struct DrinkRecipe {
    pub id: u64,
    pub drink_name: String,
    pub drink_alternate: String,
    pub tags: String,
    pub video: String,
    pub category: String,
    pub iba: String,
    pub alcoholic: String,
    pub glass: String,
    pub instructions: String,
    pub instructions_es: String,
    pub instructions_de: String,
    pub instructions_fr: String,
    pub instructions_it: String,
    pub instructions_zh_hans: String,
    pub instructions_zh_hant: String,
    pub thumb: String,
    pub ingredients: Vec<String>,
    pub image_source: String,
    pub image_attribution: String,
    pub creative_commons_confirmed: String,
    pub date_modified: String,
}
impl DrinkRecipe {
    pub fn default() -> Self {
        Self {
            id: 0,
            drink_name: "Default drink".to_string(),
            drink_alternate: "Default drink_alternate".to_string(),
            tags: "Default tags".to_string(),
            video: "Default video".to_string(),
            category: "Default category".to_string(),
            iba: "Default iba".to_string(),
            alcoholic: "Default alcoholic".to_string(),
            glass: "Default glass".to_string(),
            instructions: "Default instructions".to_string(),
            instructions_es: "Default instructions_es".to_string(),
            instructions_de: "Default instructions_de".to_string(),
            instructions_fr: "Default instructions_fr".to_string(),
            instructions_it: "Default instructions_it".to_string(),
            instructions_zh_hans: "Default instructions_zh_hans".to_string(),
            instructions_zh_hant: "Default instructions_zh_hant".to_string(),
            thumb: "Default thumb".to_string(),
            ingredients: Vec::<String>::new(),
            image_source: "Default mage_source".to_string(),
            image_attribution: "Default image_attribution".to_string(),
            creative_commons_confirmed: "Default creative_commons_confirmed".to_string(),
            date_modified: "Default date_modified".to_string(),
        }
    }

    pub fn from_drink(drink: DrinkAPI) -> Self {
        let id = match &drink.id_drink {
            Some(str_id) => str_id.clone().parse().unwrap_or_default(),
            None => 0,
        };

        let drink_name = match &drink.str_drink {
            Some(str) => str.clone(),
            None => "Default recipe name".to_string(),
        };

        let drink_alternate = match &drink.str_drink_alternate {
            Some(str) => str.clone(),
            None => "Default recipe name".to_string(),
        };

        let tags = match &drink.str_tags {
            Some(str) => str.clone(),
            None => "Default recipe name".to_string(),
        };

        let video = match &drink.str_video {
            Some(str) => str.clone(),
            None => "Default recipe name".to_string(),
        };

        let category = match &drink.str_category {
            Some(str) => str.clone(),
            None => "Default recipe name".to_string(),
        };

        let iba = match &drink.str_iba {
            Some(str) => str.clone(),
            None => "Default recipe name".to_string(),
        };
        let alcoholic = match &drink.str_alcoholic {
            Some(str) => str.clone(),
            None => "Default recipe name".to_string(),
        };
        let glass = match &drink.str_glass {
            Some(str) => str.clone(),
            None => "Default recipe name".to_string(),
        };
        let instructions = match &drink.str_instructions {
            Some(str) => str.clone(),
            None => "Default recipe name".to_string(),
        };
        let instructions_es = match &drink.str_instructions_es {
            Some(str) => str.clone(),
            None => "Default recipe name".to_string(),
        };
        let instructions_de = match &drink.str_instructions_de {
            Some(str) => str.clone(),
            None => "Default recipe name".to_string(),
        };
        let instructions_fr = match &drink.str_instructions_fr {
            Some(str) => str.clone(),
            None => "Default recipe name".to_string(),
        };
        let instructions_it = match &drink.str_instructions_it {
            Some(str) => str.clone(),
            None => "Default recipe name".to_string(),
        };
        let instructions_zh_hans = match &drink.str_instructions_zh_hans {
            Some(str) => str.clone(),
            None => "Default recipe name".to_string(),
        };
        let instructions_zh_hant = match &drink.str_instructions_zh_hant {
            Some(str) => str.clone(),
            None => "Default recipe name".to_string(),
        };
        let thumb = match &drink.str_drink_thumb {
            Some(str) => str.clone(),
            None => "Default recipe name".to_string(),
        };
        let image_source = match &drink.str_image_source {
            Some(str) => str.clone(),
            None => "Default recipe name".to_string(),
        };
        let image_attribution = match &drink.str_image_attribution {
            Some(str) => str.clone(),
            None => "Default recipe name".to_string(),
        };
        let creative_commons_confirmed = match &drink.str_creative_commons_confirmed {
            Some(str) => str.clone(),
            None => "Default recipe name".to_string(),
        };
        let date_modified = match &drink.date_modified {
            Some(str) => str.clone(),
            None => "Default recipe name".to_string(),
        };

        let ingredients = ingredients_list(&drink);
        Self {
            id,
            drink_name,
            drink_alternate,
            tags,
            video,
            category,
            iba,
            alcoholic,
            glass,
            instructions,
            instructions_es,
            instructions_de,
            instructions_fr,
            instructions_it,
            instructions_zh_hans,
            instructions_zh_hant,
            thumb,
            ingredients,
            image_source,
            image_attribution,
            creative_commons_confirmed,
            date_modified,
        }
    }
}

fn ingredients_list(drink: &DrinkAPI) -> Vec<String> {
    let mut ingredients = Vec::new();

    let mut measured_ingredient = measure_ingredient(&drink.str_measure1, &drink.str_ingredient1);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&drink.str_measure2, &drink.str_ingredient2);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&drink.str_measure3, &drink.str_ingredient3);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&drink.str_measure4, &drink.str_ingredient4);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&drink.str_measure5, &drink.str_ingredient5);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&drink.str_measure6, &drink.str_ingredient6);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&drink.str_measure7, &drink.str_ingredient7);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&drink.str_measure8, &drink.str_ingredient8);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&drink.str_measure9, &drink.str_ingredient9);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&drink.str_measure10, &drink.str_ingredient10);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&drink.str_measure11, &drink.str_ingredient11);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&drink.str_measure12, &drink.str_ingredient12);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&drink.str_measure13, &drink.str_ingredient13);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&drink.str_measure14, &drink.str_ingredient14);
    ingredients.push(measured_ingredient);

    measured_ingredient = measure_ingredient(&drink.str_measure15, &drink.str_ingredient15);
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
    let drink = crate::data::drink::get_random_drink().expect("Should be valid Drink");
    let should_be = ingredients_list(&drink);
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
