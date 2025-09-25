#!/bin/bash
curl --location 'http://localhost:8080/realms/contacts/protocol/openid-connect/token' \
--header 'Content-Type: application/x-www-form-urlencoded' \
--data-urlencode 'client_id=contacts-app-client' \
--data-urlencode 'username=testuser' \
--data-urlencode 'password=testpassword' \
--data-urlencode 'grant_type=password' \
--data-urlencode 'scope=openid email profile roles'