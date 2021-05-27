use gtk::prelude::*;
use oauth2::Client;
use oauth2::PkceCodeVerifier;
use std::thread;
use webkit2gtk::WebView;
use webkit2gtk::WebViewExt;

use oauth2::reqwest::http_client;
use oauth2::{basic::BasicClient, revocation::StandardRevocableToken, TokenResponse};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    RevocationUrl, Scope, TokenUrl,
};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use url::Url;

use firebase_handler::email_handler::login_with_email;
use firebase_handler::google_oauth;

use crate::gui::favorites_update;
use crate::gui::initialize_user;
use crate::gui_data::login_window::LoginWindow;
use crate::gui_data::GuiData;

pub fn on_email_login_button_clicked(gui_data: &GuiData, login_window: &LoginWindow) {
    let email_login_entry = login_window.email_login_entry.clone();
    let password_login_entry = login_window.password_login_entry.clone();
    let window = login_window.window.clone();

    let email = email_login_entry.get_text();
    let email = email.as_str();

    let password = password_login_entry.get_text();
    let password = password.as_str();

    let session_result = login_with_email(email, password);

    match session_result {
        Ok(session) => {
            let login_session =
                firebase_handler::write_cached_refresh_token(session.user_id.as_str());

            match login_session {
                Ok(()) => {
                    window.hide();
                    initialize_user(&gui_data);
                    favorites_update(&gui_data);
                }
                Err(e) => {
                    println!("Login session error: {}", e);
                    let login_error_label = login_window.login_error_label.clone();
                    login_error_label.set_text("Incorrect email or password");
                }
            }
        }
        Err(e) => {
            println!("Login error: {}", e);
            let login_error_label = login_window.login_error_label.clone();
            login_error_label.set_text("Incorrect email or password");
        }
    }
}

pub fn on_google_login_button_clicked(login_window: &LoginWindow) {
    let window = login_window.oauth_window.oauth_window.clone();
    let webview = WebView::new();
    let google_client_id = ClientId::new(
        include_str!("../../../../../../google_client_id")
            .trim()
            .to_string(),
    );
    let google_client_secret = ClientSecret::new(
        include_str!("../../../../../../google_client_secret")
            .trim()
            .to_string(),
    );
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .expect("Invalid token endpoint URL");

    let client = BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(
        RedirectUrl::new("http://localhost:8080".to_string()).expect("Invalid redirect URL"),
    )
    .set_revocation_uri(
        RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
            .expect("Invalid revocation endpoint URL"),
    );

    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    webview.load_uri(authorize_url.as_str());
    window.add(&webview);
    window.show_all();

    thread::spawn(move || {
        start_listener(pkce_code_verifier, csrf_state, client);
    });
}

fn start_listener(
    pkce_code_verifier: PkceCodeVerifier,
    _csrf_state: CsrfToken,
    client: Client<
        oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
        oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
        oauth2::basic::BasicTokenType,
        oauth2::StandardTokenIntrospectionResponse<
            oauth2::EmptyExtraTokenFields,
            oauth2::basic::BasicTokenType,
        >,
        StandardRevocableToken,
        oauth2::StandardErrorResponse<oauth2::RevocationErrorResponseType>,
    >,
) {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let code;
            let _state;
            {
                let mut reader = BufReader::new(&stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).unwrap();

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let (_, value) = code_pair;
                code = AuthorizationCode::new(value.into_owned());

                let state_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "state"
                    })
                    .unwrap();

                let (_, value) = state_pair;
                _state = CsrfToken::new(value.into_owned());
            }

            let token_response = client
                .exchange_code(code)
                .set_pkce_verifier(pkce_code_verifier)
                .request(http_client);

            match token_response {
                Ok(token) => {
                    let x = google_oauth(token.access_token().secret());
                    match x {
                        Ok(()) => {
                            let message = "Everything is good. Go back to application! :)";
                            let response = format!(
                                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                                message.len(),
                                message
                            );
                            stream.write_all(response.as_bytes()).unwrap();
                        }
                        Err(e) => {
                            let message = format!("Something go wrong :( Error: {}", e);
                            let response = format!(
                                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                                message.len(),
                                message
                            );
                            stream.write_all(response.as_bytes()).unwrap();
                        }
                    }
                }
                Err(e) => {
                    println!("Token respone error: {}", e)
                }
            }
            break;
        }
    }
}
