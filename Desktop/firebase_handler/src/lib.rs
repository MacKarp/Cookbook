use firestore_db_and_auth::errors::FirebaseError;
use firestore_db_and_auth::users::FirebaseAuthUserResponse;
use firestore_db_and_auth::{errors, sessions, Credentials};
use serde::{Deserialize, Serialize};

pub mod dto;
pub mod email_handler;
pub mod favorites;

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
    let refresh_token: String = match std::fs::read_to_string("token") {
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
    std::fs::write("token", &user_session.refresh_token.as_ref().unwrap())?;
    Ok(())
}

pub fn google_oauth(token: &str) -> Result<(), reqwest::Error> {
    let credentials = get_credentials();
    let uri = "https://identitytoolkit.googleapis.com/v1/accounts:signInWithIdp?key=".to_owned()
        + &credentials.api_key;

    let post_body = format!("access_token={}&providerId=google.com", token);
    let request_uri = "http://localhost".to_string();
    let return_idp_credential = true;
    let return_secure_token = true;

    let json = &SignInWithIdpRequest {
        post_body,
        request_uri,
        return_idp_credential,
        return_secure_token,
    };

    let response = reqwest::blocking::Client::new()
        .post(&uri)
        .json(&json)
        .send()?;

    let oauth_response: OAuthResponse = response.json()?;

    write_cached_refresh_token(&oauth_response.local_id).unwrap();
    Ok(())
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
struct SignInWithIdpRequest {
    pub post_body: String,
    pub request_uri: String,
    pub return_idp_credential: bool,
    pub return_secure_token: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuthResponse {
    pub federated_id: String,
    pub provider_id: String,
    pub local_id: String,
    pub email_verified: bool,
    pub email: String,
    pub oauth_access_token: String,
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub display_name: String,
    pub id_token: String,
    pub photo_url: String,
    pub refresh_token: String,
    pub expires_in: String,
    pub raw_user_info: String,
}
