# Docker Setup

## Docker Files

- `Dockerfile` : Multi-stage optimized build with dependency caching
- `docker-compose.yml` : Development setup that connects to existing Keycloak

## Usage

### Local Development (with existing Keycloak)

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

## Useful Commands

### View logs
```bash
docker compose logs -f user-api
```

### Rebuild after changes
```bash
docker compose up -d --build
```

### Stop services
```bash
docker compose down
```

### Stop and remove volumes
```bash
docker compose down -v
```

## Network Architecture

```
┌─────────────────────────────────────┐
│   keycloak_network (external)      │
│   ┌──────────┐      ┌───────────┐  │
│   │ Keycloak │◄─────┤ User API  │  │
│   └──────────┘      └─────┬─────┘  │
└───────────────────────────┼────────┘
                            │
┌───────────────────────────┼────────┐
│      user-network         │        │
│   ┌───────────┐      ┌────▼────┐  │
│   │ Postgres  │◄─────┤ User API│  │
│   └───────────┘      └─────────┘  │
└────────────────────────────────────┘
```

The user-api service is connected to two networks:
- `keycloak_network` : to communicate with Keycloak
- `user-network` : to communicate with its own database

## Dockerfile Optimizations

The Dockerfile uses several optimizations:
1. **Multi-stage build** : Separates compilation from runtime image
2. **Dependency caching** : Builds dependencies before source code
3. **Minimal image** : Uses debian:bookworm-slim for runtime
4. **Non-root user** : Runs the application with a dedicated user
5. **Migrations included** : SQL migrations are copied into the image

## Troubleshooting

### Service cannot connect to Keycloak
Check that the `keycloak_network` network exists and Keycloak is running:
```bash
docker network ls | grep keycloak
docker compose -f /path/to/client/docker-compose.yml ps
```

### Database connection error
Check that PostgreSQL is running and accessible:
```bash
docker compose logs postgres
```

### Check migration status
```bash
docker compose run --rm user-api migrate
```
