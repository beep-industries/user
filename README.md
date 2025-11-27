# User Service

Rust microservice for user management.

## Local Development (with existing Keycloak)

1. **Start Keycloak from your client project** :
```bash
cd /path/to/your/client
docker compose up -d
```

2. **Configure the Keycloak client** :
   - Login to Keycloak: http://localhost:8080
   - Create the `user-service` client in the `myrealm` realm
   - Enable "Client authentication" (confidential)
   - Enable "Service accounts roles"
   - Get the secret from the "Credentials" tab

3. **Create the `.env` file** :
```bash
cp .env.example .env
```

Edit `.env` :
```env
KEYCLOAK_CLIENT_SECRET=your-secret-from-keycloak
JWT_SECRET=your-jwt-secret-key
```

4. **Start postgres and run migrations** :
```bash
docker compose up -d postgres
docker compose run --rm user-api migrate
```

5. **Start the user service** :
```bash
docker compose up -d user-api
```

The service will be available at http://localhost:3000

## CLI Commands

The `user-api` binary supports the following commands:

| Command | Description |
|---------|-------------|
| `migrate` | Run database migrations |
| `run` | Start the API server (default) |

### Run migrations
```bash
docker compose up -d postgres
docker compose run --rm user-api migrate
```

### Start the server
```bash
docker compose up -d
# or explicitly:
docker compose run --rm user-api run
```

## Manual Build

### Build the image
```bash
docker build -t user-service:latest .
```

### Run the container
```bash
docker run -p 3000:3000 \
  -e DATABASE_URL=postgres://... \
  -e KEYCLOAK_URL=http://keycloak:8080 \
  -e KEYCLOAK_REALM=myrealm \
  -e KEYCLOAK_CLIENT_ID=user-service \
  -e KEYCLOAK_CLIENT_SECRET=your-secret \
  -e JWT_SECRET=your-jwt-secret \
  user-service:latest
```
