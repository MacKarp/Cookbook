# How to build:
Generate and download the service accounts credentials file as JSON format from [Google Cloud Service Account](https://console.cloud.google.com/iam-admin/serviceaccounts) and store it as `"firebase-service-account.json"`.

Open `firebase-service-account.json` and add another field `"api_key" : "YOUR_API_KEY"` and replace `YOUR_API_KEY` with your *Web API key*, to be found in the [Google Firebase console](https://console.firebase.google.com) in "Project Overview -> Settings - > General".

Download the 2 public key files and put them into `firebase_handler` directory:

* https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com -> Store as `securetoken.jwks`
* https://www.googleapis.com/service_accounts/v1/jwk/{your-service-account-email} -> Store as `service-account.jwks`

