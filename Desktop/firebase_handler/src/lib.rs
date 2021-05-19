use firestore_db_and_auth::sessions::user::Session;
use firestore_db_and_auth::Credentials;

pub mod email_handler;

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

pub fn get_user_info(session: &Session) {
    let info = firestore_db_and_auth::users::user_info(session).unwrap();
    let a = info.users;
    println!("test: {:#?}", a);
}
