use firestore_db_and_auth::{errors, sessions, users, Credentials};

pub mod email_handler;
pub mod favorites;

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

pub fn get_user_info(session: &sessions::user::Session) {
    let info = users::user_info(session).unwrap();
    let a = info.users;
    println!("test: {:#?}", a);
}

pub fn user_session_with_cached_refresh_token(
    cred: &Credentials,
    user_id: &str,
) -> errors::Result<sessions::user::Session> {
    println!("Refresh token from file");
    // Read refresh token from file if possible instead of generating a new refresh token each time
    let refresh_token: String = match std::fs::read_to_string("refresh-token-for-tests.txt") {
        Ok(v) => v,
        Err(e) => {
            if e.kind() != std::io::ErrorKind::NotFound {
                return Err(errors::FirebaseError::IO(e));
            }
            String::new()
        }
    };

    // Generate a new refresh token if necessary
    println!("Generate new user auth token");
    let user_session: sessions::user::Session = if refresh_token.is_empty() {
        let session = sessions::user::Session::by_user_id(&cred, user_id, true)?;
        std::fs::write(
            "refresh-token-for-tests.txt",
            &session.refresh_token.as_ref().unwrap(),
        )?;
        session
    } else {
        println!("user::Session::by_refresh_token");
        sessions::user::Session::by_refresh_token(&cred, &refresh_token)?
    };

    Ok(user_session)
}
