use crate::models::drink::filter::AllDrinkFilteredApi;

#[derive(Debug)]
pub struct DrinkFilteredList {
    pub filtered_drinks: Vec<Drink>,
}
impl DrinkFilteredList {
    pub fn default() -> Self {
        let filtered_drinks = Vec::<Drink>::new();
        Self { filtered_drinks }
    }

    pub fn from_api(drink_list: AllDrinkFilteredApi) -> Self {
        let mut filtered_drinks = Vec::<Drink>::new();

        for d in drink_list.drinks {
            let id_drink = match d.id_drink {
                Some(s) => s,
                None => String::from("0"),
            };
            let str_drink = match d.str_drink {
                Some(s) => s,
                None => String::default(),
            };
            let str_drink_thumb = match d.str_drink_thumb {
                Some(s) => s,
                None => String::default(),
            };
            let drink = Drink {
                id_drink,
                str_drink,
                str_drink_thumb,
            };
            filtered_drinks.push(drink);
        }

        Self { filtered_drinks }
    }
}

#[derive(Debug)]
pub struct Drink {
    pub str_drink: String,
    pub str_drink_thumb: String,
    pub id_drink: String,
}
