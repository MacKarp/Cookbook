# How to build:
Download the 2 public key files and put them into `firebase_handler` directory:

* https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com -> Store as `securetoken.jwks`
* https://www.googleapis.com/service_accounts/v1/jwk/{your-service-account-email} -> Store as `service-account.jwks`

Download also the service accounts credentials file and store it as "firebase-service-account.json".
Add another field `"api_key" : "YOUR_API_KEY"` and replace `YOUR_API_KEY` with your *Web API key*, to be found in the [Google Firebase console](https://console.firebase.google.com) in "Project Overview -> Settings - > General".