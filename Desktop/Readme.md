# Requiermentrs: 
To get best experience using app following fonts are required to be installed:

[Great Vibes](https://fonts.google.com/specimen/Great+Vibes)

[Poppins](https://fonts.google.com/specimen/Poppins)

# How to build:
Generate and download the service accounts credentials file as JSON format from [Google Cloud Service Account](https://console.cloud.google.com/iam-admin/serviceaccounts) and store it as `firebase-service-account.json`.

Open `firebase-service-account.json` and add another field `"api_key" : "YOUR_API_KEY"` and replace `YOUR_API_KEY` with your *Web API key*, to be found in the [Google Firebase console](https://console.firebase.google.com) in "Project Overview -> Settings - > General".

Download the 2 public key files and put them into `firebase_handler` directory:

* https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com -> Save as `securetoken.jwks`
* https://www.googleapis.com/service_accounts/v1/jwk/{your-service-account-email} -> Save as `service-account.jwks`

Go to [Credentials – APIs & Services – Cookbook – Google Cloud Platform](https://console.cloud.google.com/apis/credentials) Add new *OAuth 2.0 Client IDs* and save `Client ID` as -> `google_client_id` and `Client secret` as ->`google_client_secret`

Go to [Facebook Credentials]((https://facebook.com)) Add new *OAuth 2.0 Client IDs* and save `Client ID` as -> `facebook_client_id` and `Client secret` as ->`facebook_client_secret`