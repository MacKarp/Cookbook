use cookbook::dto::meal::recipe::MealRecipe;
use firestore_db_and_auth::dto::{Document, FieldOperator};
use firestore_db_and_auth::errors;
use firestore_db_and_auth::sessions::user::Session;
use firestore_db_and_auth::{documents::*, errors::FirebaseError};
use rand::Rng;
use std::collections::HashMap;
use std::thread;

use cookbook::dto::drink::recipe::DrinkRecipe;

use crate::dto::drink::FavoriteDrink;
use crate::dto::meal::FavoriteMeal;

pub fn query_favorites() -> errors::Result<Vec<Document>> {
    let user_session = super::read_cached_refresh_token()?;
    let user = firestore_db_and_auth::users::user_info(&user_session).unwrap();
    let user = &user.users[0];
    let dispaly_name = user.displayName.clone().unwrap();
    let results: Vec<Document> = query(
        &user_session,
        "favorites",
        dispaly_name.into(),
        FieldOperator::EQUAL,
        "userName",
    )
    .unwrap()
    .collect();

    Ok(results)
}

pub fn get_favorites() -> (
    HashMap<String, FavoriteMeal>,
    HashMap<String, FavoriteDrink>,
) {
    let results = query_favorites();
    let mut favorites_uid = vec![];
    if let Ok(d) = results {
        for document in d {
            favorites_uid.push(document.name.replace(
                "projects/cookbook-307109/databases/(default)/documents/favorites/",
                "",
            ));
        }
    }
    (
        get_favorite_meal_recipe(&favorites_uid),
        get_favorite_drink_recipe(&favorites_uid),
    )
}

fn get_favorite_drink_recipe(uid: &[String]) -> HashMap<String, FavoriteDrink> {
    let sesion = super::read_cached_refresh_token();
    let mut drink_recipes = HashMap::new();
    if let Ok(s) = sesion {
        let path = "favorites";
        for id in uid {
            let drink: FavoriteDrink =
                firestore_db_and_auth::documents::read(&s, path, id).unwrap();
            if drink.drink_name.is_some() {
                drink_recipes.insert(id.clone(), drink);
            }
        }
    }
    drink_recipes
}

pub fn get_favorite_meal_recipe(uid: &[String]) -> HashMap<String, FavoriteMeal> {
    let sesion = super::read_cached_refresh_token();
    let mut meal_recipes = HashMap::new();
    if let Ok(s) = sesion {
        let path = "favorites";
        for id in uid {
            let meal: FavoriteMeal = firestore_db_and_auth::documents::read(&s, path, id).unwrap();
            if meal.meal_name.is_some() {
                meal_recipes.insert(id.clone(), meal);
            }
        }
    }
    meal_recipes
}
pub fn save_favorite_meal_recipe(meal_recipe: MealRecipe) -> Result<WriteResult, FirebaseError> {
    let thread = thread::spawn(|| {
        let sesion = super::read_cached_refresh_token();
        match sesion {
            Ok(s) => {
                let user_id = s.user_id.clone();
                let user = super::get_user_info().unwrap();
                let user_name = user.users[0].displayName.clone().unwrap_or_default();

                let path = "favorites";
                let document_id = Some(generate_random_id());
                let user_info = (&user_id, &user_name);

                let document = FavoriteMeal::from_meal_recipe(meal_recipe, user_info);
                write(&s, path, document_id, &document, WriteOptions::default())
            }
            Err(e) => Err(e),
        }
    });
    thread.join().unwrap()
}
pub fn save_favorite_drink_recipe(drink_recipe: DrinkRecipe) -> Result<WriteResult, FirebaseError> {
    let thread = thread::spawn(|| {
        let sesion = super::read_cached_refresh_token();
        match sesion {
            Ok(s) => {
                let user_id = s.user_id.clone();
                let user = super::get_user_info().unwrap();
                let user_name = user.users[0].displayName.clone().unwrap_or_default();

                let path = "favorites";
                let document_id = Some(generate_random_id());
                let user_info = (&user_id, &user_name);

                let document = FavoriteDrink::from_drink_recipe(drink_recipe, user_info);
                write(&s, path, document_id, &document, WriteOptions::default())
            }
            Err(e) => Err(e),
        }
    });
    thread.join().unwrap()
}
pub fn remove_favorite_recipe(document_id: &str) -> Result<(), FirebaseError> {
    let sesion = super::read_cached_refresh_token();
    match sesion {
        Ok(s) => {
            let path = "favorites/".to_owned() + document_id;
            delete(&s, &path, false)
        }
        Err(e) => Err(e),
    }
}

fn generate_random_id() -> String {
    let sesion = super::read_cached_refresh_token();
    let mut all_favorites_id = vec![];
    if let Ok(s) = sesion {
        let list: List<FavoriteMeal, Session> =
            firestore_db_and_auth::documents::list(&s, "favorites");
        for document in list {
            let result = document.unwrap();
            all_favorites_id.push(result.1.name.clone().replace(
                "projects/cookbook-307109/databases/(default)/documents/favorites/",
                "",
            ));
        }
    }
    loop {
        let uid = rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .map(char::from)
            .take(20)
            .collect();
        if !all_favorites_id.contains(&uid) {
            return uid;
        }
    }
}
