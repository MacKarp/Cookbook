use firestore_db_and_auth::documents::*;
use firestore_db_and_auth::dto::{Document, FieldOperator};
use firestore_db_and_auth::errors;

use serde::{Deserialize, Serialize};

pub fn query_favorites() -> errors::Result<Vec<Document>> {
    let user_session = super::read_cached_refresh_token()?;
    let ff = firestore_db_and_auth::users::user_info(&user_session).unwrap();
    println!("test: {:#?}", &ff.users);
    let dd = ff.users[0].displayName.clone().unwrap();
    let results: Vec<Document> = query(
        &user_session,
        "favorites",
        dd.into(),
        FieldOperator::EQUAL,
        "userName",
    )
    .unwrap()
    .collect();

    Ok(results)
}

pub fn get_favorites() -> Vec<Favorite> {
    let results = query_favorites();
    let mut favorites_uid = vec![];
    match results {
        Ok(d) => {
            for document in d {
                favorites_uid.push(document.name.replace(
                    "projects/cookbook-307109/databases/(default)/documents/favorites/",
                    "",
                ))
            }
        }
        Err(e) => println!("Favorites error: {}", e),
    }

    get_favorite_recipe(favorites_uid)
}

pub fn get_favorite_recipe(uid: Vec<String>) -> Vec<Favorite> {
    let sesion = super::read_cached_refresh_token();
    let mut recipes = vec![];
    match sesion {
        Ok(s) => {
            let path = "favorites";
            for id in uid {
                let x: Favorite = firestore_db_and_auth::documents::read(&s, path, id).unwrap();
                recipes.push(x);
            }
        }
        Err(e) => println!("Session error: {}", e),
    }

    recipes
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Favorite {
    pub created_at: Option<String>,
    pub meal_area: Option<String>,
    pub meal_category: Option<String>,
    pub meal_id: Option<String>,
    pub meal_name: Option<String>,
    pub meal_photo_url: Option<String>,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
}
