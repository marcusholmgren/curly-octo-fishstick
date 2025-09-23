# curly-octo-fishstick

Rust HTTP Server and API for managing contacts

## Database migrations

Using Diesel CLI

```bash
diesel migration run
```


## API

Get all contacts
```bash
curl http://127.0.0.1:8080/api/contacts
```

Get contact by id
```bash
curl http://127.0.0.1:8080/api/contacts/1
```

Create contact
```bash
curl http://127.0.0.1:8080/api/contacts -X POST -H "Content-Type: application/json" -d '{"first_name": "John", "last_name": "Doe", "email": "john.doe@example.com", "phone_number": "123456"}'
```

Update contact
```bash
curl http://127.0.0.1:8080/api/contacts/1 -X PUT -H "Content-Type: application/json" -d '{"first_name": "Jane", "last_name": "Doe", "email": "jane.doe@example.com", "phone_number": "654321"}'
```

Delete contact
```bash
curl http://127.0.0.1:8080/api/contacts/1 -X DELETE
```
