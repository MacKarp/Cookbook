use std::vec;

use firestore_db_and_auth::documents::*;
use firestore_db_and_auth::dto::{Document, FieldOperator};
use firestore_db_and_auth::errors;
use firestore_db_and_auth::sessions::user::Session;
use rand::Rng;
use serde::{Deserialize, Serialize};

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

pub fn get_favorites() -> Vec<FavoriteMeal> {
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
    get_favorite_recipe(favorites_uid)
}

pub fn get_favorite_recipe(uid: Vec<String>) -> Vec<FavoriteMeal> {
    let sesion = super::read_cached_refresh_token();
    let mut recipes = vec![];
    if let Ok(s) = sesion {
        let path = "favorites";
        for id in uid {
            let x: FavoriteMeal = firestore_db_and_auth::documents::read(&s, path, id).unwrap();
            recipes.push(x);
        }
    }

    recipes
}
//favorite: FavoriteMeal
pub fn save_favorite_meal_recipe() {
    let sesion = super::read_cached_refresh_token();
    if let Ok(_s) = sesion {
        let _path = "favorites";
        let id = Some(generate_random_id());
        println!("uid: {:?}", id);
        //let x = firestore_db_and_auth::documents::write(&s, path, id, document, options);
    };
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FavoriteMeal {
    pub created_at: Option<String>,
    pub meal_area: Option<String>,
    pub meal_category: Option<String>,
    pub meal_id: Option<String>,
    pub meal_name: Option<String>,
    pub meal_photo_url: Option<String>,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
}
