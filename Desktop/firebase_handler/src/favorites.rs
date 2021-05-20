use firestore_db_and_auth::documents::*;
use firestore_db_and_auth::dto::{Document, FieldOperator};
use firestore_db_and_auth::FirebaseAuthBearer;
use firestore_db_and_auth::{errors, sessions};

pub fn get_favorites(user_id: &str) -> errors::Result<()> {
    let cred = super::get_credentials();

    let user_session = super::user_session_with_cached_refresh_token(&cred, user_id)?;
    let user_session =
        sessions::user::Session::by_access_token(&cred, &user_session.access_token_unchecked())?;

    let results: Vec<Document> = query(
        &user_session,
        "favorites",
        "Łukasz Kuczma".into(),
        FieldOperator::EQUAL,
        "userName",
    )
    .unwrap()
    .collect();

    println!("Test: get favorites {:#?}", results);
    Ok(())
}
