# User Service

Rust microservice for user management with Keycloak integration.

## Architecture Overview

```mermaid
graph TB
    subgraph "Client Layer"
        Frontend[Frontend App<br/>React]
    end

    subgraph "Authentication Layer"
        Keycloak[Keycloak<br/>Auth Server]
        KeycloakDB[(Keycloak DB<br/>PostgreSQL<br/><br/>- Users<br/>- Auth data<br/>- Email<br/>- First name<br/>- Last name)]
    end

    subgraph "Application Layer"
        UserService[User Service<br/>Rust API]
        UserDB[(User Service DB<br/>PostgreSQL<br/><br/>- Users<br/>- Display name<br/>- Profile picture<br/>- Status<br/>- Settings)]
    end

    Frontend -->|1. Auth requests| Keycloak
    Frontend -->|3. API calls + JWT| UserService
    Keycloak -->|2. JWT tokens| Frontend
    Keycloak <-->|Stores auth data| KeycloakDB
    UserService <-->|Admin API<br/>manage users| Keycloak
    UserService <-->|Stores app data| UserDB

    style Frontend fill:#e1f5ff
    style Keycloak fill:#fff4e6
    style KeycloakDB fill:#fff4e6
    style UserService fill:#e8f5e9
    style UserDB fill:#e8f5e9
```

## Registration & Authentication Flow

```mermaid
sequenceDiagram
    participant U as User
    participant F as Frontend
    participant K as Keycloak
    participant US as User Service
    participant KDB as Keycloak DB
    participant USDB as User Service DB

    Note over U,USDB: 1. User Registration in Keycloak
    U->>F: Register (email, password, name)
    F->>K: Redirect to Keycloak<br/>registration page
    U->>K: Fill registration form
    K->>KDB: INSERT user<br/>(sub, email, first_name, last_name)
    KDB-->>K: User created
    K-->>F: Redirect with auth code
    F->>K: Exchange code for JWT token
    K-->>F: JWT token (with sub)

    Note over U,USDB: 2. First API Call (Auto-create or Login)
    F->>US: GET /users/me<br/>(Bearer token)
    US->>US: Validate JWT<br/>Extract sub from token
    US->>USDB: SELECT user<br/>WHERE sub = ?

    alt User doesn't exist (first login)
        USDB-->>US: User not found
        US->>USDB: INSERT user<br/>(sub, status=active)
        USDB-->>US: User created
        US-->>F: User data<br/>(id, sub, display_name, status)
        F-->>U: Account created ✓
    else User exists (returning user)
        USDB-->>US: User found
        US-->>F: User data<br/>(id, sub, display_name, status)
        F-->>U: Logged in ✓
    end

    Note over U,USDB: 3. Update Profile (Local + Keycloak fields)
    U->>F: Update profile<br/>(last_name, display_name)
    F->>US: PUT /users/me<br/>{last_name, display_name}
    US->>US: Validate JWT

    alt Keycloak field (last_name)
        US->>K: Admin API:<br/>PUT /users/{sub}
        K->>KDB: UPDATE user<br/>SET last_name = ?
        KDB-->>K: Updated
        K-->>US: Success
    end

    alt Local field (display_name)
        US->>USDB: UPDATE users<br/>SET display_name = ?
        USDB-->>US: Updated
    end

    US-->>F: Updated user data
    F-->>U: Profile updated ✓
```

## Pre-commit Hooks

First, install pre-commit following the [official installation guide](https://pre-commit.com/#install).

Then, install the hooks in your repository:

```bash
pre-commit install
```

### Usage

The hooks run automatically before each `git commit`. To run them manually:

```bash
# Run on all files
pre-commit run --all-files

# Run on staged files only
pre-commit run

# Run a specific hook
pre-commit run fmt
pre-commit run clippy
```

## Local Development

This project uses Keycloak from the [beep-industries/client](https://github.com/beep-industries/client) repository for local development.

### Prerequisites

Clone and start the client repository (includes Keycloak with pre-configured realm):

```bash
git clone https://github.com/beep-industries/client.git
cd client
docker compose up -d
```

Keycloak will be available at http://localhost:8080 with the `myrealm` realm already configured, including:
- `frontend` client for React application
- `user-service` client for backend API (with service account)
- Test user: `testuser` / `test123`

### Docker Networks

The project uses two Docker networks:

- **`user-network`**: Internal bridge network for communication between the user-api and its PostgreSQL database
- **`keycloak_network`**: External network shared with the client project to communicate with Keycloak. This network must be created by starting the client's docker-compose first.

This is why two Keycloak URLs are configured:
- `KEYCLOAK_URL`: External URL (http://localhost:8080) for browser redirects
- `KEYCLOAK_INTERNAL_URL`: Internal URL (http://client-keycloak-1:8080) for service-to-service communication via the Docker network

### Setup User Service

1. **Create `.env` file**:
```bash
cp .env.example .env
```

2. **Start the user service**:
```bash
# Start all services (postgres + user-api)
docker compose up -d

# Or start services independently:
docker compose up -d postgres              # Start database only
docker compose run --rm user-api migrate   # Run migrations
docker compose up -d user-api              # Start API only
```

The service will be available at http://localhost:3000

### API Documentation

Interactive API documentation is available via Scalar at:

**http://localhost:3000/docs**

## CLI Commands

The `user-api` binary supports the following commands:

| Command | Description |
|---------|-------------|
| `migrate` | Run database migrations |
| `run` | Start the API server (default) |

## Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | `postgres://user:pass@host:5432/db` |
| `SERVER_HOST` | Server host | `0.0.0.0` |
| `SERVER_PORT` | Server port | `3000` |
| `KEYCLOAK_URL` | Keycloak external URL (for browser) | `http://localhost:8080` |
| `KEYCLOAK_INTERNAL_URL` | Keycloak internal URL (for service) | `http://client-keycloak-1:8080` |
| `KEYCLOAK_REALM` | Keycloak realm name | `myrealm` |
| `KEYCLOAK_CLIENT_ID` | Keycloak client ID | `user-service` |
| `KEYCLOAK_CLIENT_SECRET` | Keycloak client secret | `ABvykyIUah2CcQPiRcvcgd7GA4MrEdx4` |
| `JWT_SECRET` | JWT secret (unused with RS256) | `secret` |
