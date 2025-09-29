# Contacts Application

This is a full-stack web application for managing contacts. It features a backend API built with Rust and Actix-web, and a frontend client built with SvelteKit. The application uses Keycloak for authentication.

## Table of Contents

- [Architecture](#architecture)
- [Tech Stack](#tech-stack)
- [Prerequisites](#prerequisites)
- [Getting Started](#getting-started)
  - [1. Environment Variables](#1-environment-variables)
  - [2. Running the Application](#2-running-the-application)
  - [3. Database Setup](#3-database-setup)
- [Usage](#usage)
  - [Accessing the Application](#accessing-the-application)
  - [Test User](#test-user)

## Architecture

The application is composed of three main services, orchestrated using Docker Compose:

- **`frontend`**: A SvelteKit application that provides the user interface for managing contacts. It communicates with the backend API.
- **`backend`**: A Rust-based API built with Actix-web. It handles the business logic and database interactions for the contacts.
- **`keycloak`**: An open-source identity and access management solution used for user authentication.

## Tech Stack

- **Backend**: Rust, Actix-web, Diesel (for ORM), SQLite (for the database)
- **Frontend**: SvelteKit, TypeScript, Tailwind CSS
- **Authentication**: Keycloak (via Auth.js)
- **Containerization**: Docker, Docker Compose

## Prerequisites

Before you begin, ensure you have the following installed on your system:

- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)
- [Diesel CLI](https://diesel.rs/guides/getting-started) (`cargo install diesel_cli --no-default-features --features sqlite`)

## Getting Started

Follow these steps to get the application up and running locally.

### 1. Environment Variables

The application uses environment variables for configuration. Create a `.env` file in the root of the project with the following content:

```env
# Backend Configuration
DATABASE_URL=sqlite://contacts.db
IDP_URL=http://localhost:8080/realms/contacts
IDP_AUDIENCE=contacts-app-client

# Frontend Configuration
AUTH_SECRET=a_very_secret_and_long_string_for_auth_js
AUTH_KEYCLOAK_SECRET=your_keycloak_client_secret
AUTH_KEYCLOAK_ID=contacts-app-client
AUTH_KEYCLOAK_ISSUER=http://localhost:8080/realms/contacts
AUTH_TRUST_HOST=true
CONTACTS_API=http://localhost:8081
AUTH_URL=http://localhost:3000/auth
AUTH_ORIGIN=http://localhost:3000
```

**Note:** You will need to get the `AUTH_KEYCLOAK_SECRET` from the Keycloak admin console after the Keycloak service has started. Go to `http://localhost:8080`, log in with `admin`/`admin_password`, navigate to the `contacts` realm, select the `contacts-app-client` client, and find the secret in the "Credentials" tab.

### 2. Running the Application

Start all the services using Docker Compose:

```bash
docker-compose up --build
```

This command will build the images for the frontend and backend services and start all three containers.

### 3. Database Setup

Once the containers are running, you need to set up the database and run the migrations for the backend service.

In a new terminal window, execute the following command to run the Diesel migrations inside the running `backend` container:

```bash
docker-compose exec backend diesel migration run
```

Your application should now be fully set up and running.

## Usage

### Accessing the Application

- **Frontend**: Open your browser and navigate to `http://localhost:3000`
- **Backend API**: The API is available at `http://localhost:8081`
- **Keycloak Admin Console**: Access it at `http://localhost:8080`

### Test User

A test user is pre-configured in the Keycloak realm. You can use these credentials to sign in to the application:

- **Username**: `testuser`
- **Password**: `testpassword`