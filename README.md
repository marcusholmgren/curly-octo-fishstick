# curly-octo-fishstick

Application with a backend API for storing contacts and a frontend for managing contacts.
Users must login to access the application.


Get a token from your IDP
```bash
curl --location 'http://localhost:8080/realms/contacts/protocol/openid-connect/token' \
--header 'Content-Type: application/x-www-form-urlencoded' \
--data-urlencode 'client_id=contacts-app-client' \
--data-urlencode 'username=testuser' \
--data-urlencode 'password=testpassword' \
--data-urlencode 'grant_type=password'
```
