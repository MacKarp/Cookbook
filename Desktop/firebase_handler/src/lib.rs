use firestore_db_and_auth::Credentials;

pub fn create_with_email(email: &str, password: &str) {
    let session =
        firestore_db_and_auth::sessions::service_account::Session::new(get_credentials()).unwrap();

    let response = firestore_db_and_auth::users::sign_up(&session, email, password);
    let register_sesion = response.unwrap();
    let api_key = register_sesion.api_key;
    let user_id = register_sesion.user_id;
    let refresh_token = register_sesion.refresh_token;
    println!(
        "Session create:\napi_key: {:?}\nuser_id: {:?}\nrefresh_token: {:?}",
        api_key, user_id, refresh_token
    );
}

pub fn login_with_email(email: &str, password: &str) {
    let session =
        firestore_db_and_auth::sessions::service_account::Session::new(get_credentials()).unwrap();

    let response = firestore_db_and_auth::users::sign_in(&session, email, password);
    let login_session = response.unwrap();
    let api_key = login_session.api_key;
    let user_id = login_session.user_id;
    let refresh_token = login_session.refresh_token;
    println!(
        "Session login:\napi_key: {:?}\nuser_id: {:?}\nrefresh_token: {:?}",
        api_key, user_id, refresh_token
    );
}

fn get_credentials() -> Credentials {
    Credentials::new(
        include_str!("../firebase-service-account.json"),
        &[
            include_str!("../securetoken.jwks"),
            include_str!("../service-account.jwks"),
        ],
    )
    .unwrap()
}
