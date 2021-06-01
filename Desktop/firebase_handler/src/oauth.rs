use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct SignInWithIdpRequest {
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
