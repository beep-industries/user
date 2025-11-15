# Docker Setup

## Fichiers Docker

- `Dockerfile` : Build multi-stage optimisé avec cache des dépendances
- `docker-compose.yml` : Setup production sans Keycloak
- `docker-compose.local.yml` : Setup développement qui se connecte au Keycloak existant

## Utilisation

### Développement Local (avec Keycloak existant)

1. **Démarrer le Keycloak depuis votre projet client** :
```bash
cd /path/to/your/client
docker compose up -d
```

2. **Configurer le client Keycloak** :
   - Connectez-vous à Keycloak : http://localhost:8080
   - Créez le client `user-service` dans le realm `myrealm`
   - Activez "Client authentication" (confidential)
   - Activez "Service accounts roles"
   - Récupérez le secret dans l'onglet "Credentials"

3. **Créer le fichier `.env`** :
```bash
cp .env.example .env
```

Éditez `.env` :
```env
KEYCLOAK_CLIENT_SECRET=votre-secret-depuis-keycloak
JWT_SECRET=votre-jwt-secret-key
```

4. **Lancer le service user** :
```bash
docker compose -f docker-compose.local.yml up -d
```

Le service sera disponible sur http://localhost:3000

### Production (sans Keycloak)

Pour la production, Keycloak doit être déployé séparément :

```bash
docker compose up -d
```

Vous devrez configurer les variables d'environnement pour pointer vers votre Keycloak :
```bash
export KEYCLOAK_URL=https://your-keycloak-domain
export KEYCLOAK_CLIENT_SECRET=your-production-secret
export JWT_SECRET=your-production-jwt-secret

docker compose up -d
```

## Build manuel

### Build l'image
```bash
docker build -t user-service:latest .
```

### Run le container
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

## Commandes utiles

### Voir les logs
```bash
docker compose -f docker-compose.local.yml logs -f user-api
```

### Rebuild après modifications
```bash
docker compose -f docker-compose.local.yml up -d --build
```

### Arrêter les services
```bash
docker compose -f docker-compose.local.yml down
```

### Arrêter et supprimer les volumes
```bash
docker compose -f docker-compose.local.yml down -v
```

## Architecture des réseaux

### Développement Local
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

Le service user-api est connecté à deux réseaux :
- `keycloak_network` : pour communiquer avec Keycloak
- `user-network` : pour communiquer avec sa propre base de données

## Optimisations du Dockerfile

Le Dockerfile utilise plusieurs optimisations :
1. **Build multi-stage** : Sépare la compilation de l'image runtime
2. **Cache des dépendances** : Build les dépendances avant le code source
3. **Image minimale** : Utilise debian:bookworm-slim pour le runtime
4. **Non-root user** : Execute l'application avec un utilisateur dédié
5. **Migrations incluses** : Les migrations SQL sont copiées dans l'image

## Troubleshooting

### Le service ne peut pas se connecter à Keycloak
Vérifiez que le réseau `keycloak_network` existe et que Keycloak est démarré :
```bash
docker network ls | grep keycloak
docker compose -f /path/to/client/docker-compose.yml ps
```

### Erreur de connexion à la base de données
Vérifiez que PostgreSQL est démarré et accessible :
```bash
docker compose -f docker-compose.local.yml logs postgres
```

### Voir les migrations
```bash
docker compose -f docker-compose.local.yml exec user-api sqlx migrate info
```
