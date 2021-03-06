use firestore_db_and_auth::errors::FirebaseError;
use firestore_db_and_auth::users::FirebaseAuthUserResponse;
use firestore_db_and_auth::{errors, sessions, Credentials};

pub mod dto;
pub mod email_auth_handler;
pub mod facebook_auth_handler;
pub mod favorites;
pub mod google_auth_handler;
pub mod oauth;

pub fn get_credentials() -> Credentials {
    Credentials::new(
        include_str!("../../firebase-service-account.json"),
        &[
            include_str!("../../securetoken.jwks"),
            include_str!("../../service-account.jwks"),
        ],
    )
    .unwrap()
}

pub fn get_user_info() -> Result<FirebaseAuthUserResponse, FirebaseError> {
    let session = match read_cached_refresh_token() {
        Ok(s) => s,
        Err(e) => {
            return Err(e);
        }
    };
    let users = firestore_db_and_auth::users::user_info(&session)?;
    Ok(users)
}

pub fn read_cached_refresh_token() -> errors::Result<sessions::user::Session> {
    let refresh_token: String = match std::fs::read_to_string(".token") {
        Ok(v) => v,
        Err(e) => {
            if e.kind() != std::io::ErrorKind::NotFound {
                return Err(errors::FirebaseError::IO(e));
            }
            return Err(errors::FirebaseError::Generic("Refresh token not found"));
        }
    };

    let credentials = get_credentials();
    let user_session = sessions::user::Session::by_refresh_token(&credentials, &refresh_token)?;
    Ok(user_session)
}

pub fn write_cached_refresh_token(user_id: &str) -> errors::Result<()> {
    let credentials = get_credentials();
    let user_session = sessions::user::Session::by_user_id(&credentials, user_id, true)?;
    std::fs::write(".token", &user_session.refresh_token.as_ref().unwrap())?;
    Ok(())
}
