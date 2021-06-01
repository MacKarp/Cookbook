use firestore_db_and_auth::errors::FirebaseError;
use firestore_db_and_auth::sessions::service_account::Session;
use firestore_db_and_auth::users::{sign_in, sign_up};
use firestore_db_and_auth::UserSession;

pub fn create_with_email(email: &str, password: &str) -> Result<UserSession, FirebaseError> {
    let session = Session::new(super::get_credentials()).unwrap();

    sign_up(&session, email, password)
}

pub fn login_with_email(email: &str, password: &str) -> Result<UserSession, FirebaseError> {
    let session = Session::new(super::get_credentials()).unwrap();

    sign_in(&session, email, password)
}
