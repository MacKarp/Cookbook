use crate::get_credentials;
use crate::oauth::OAuthResponse;
use crate::oauth::SignInWithIdpRequest;
use crate::write_cached_refresh_token;

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
