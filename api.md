# API Documentation - User Service

Base URL: `http://localhost:3000`

## Authentication

All endpoints require a **JWT Bearer Token** in the `Authorization` header.

```
Authorization: Bearer <jwt_token>
```

---

## Endpoints

### GET `/users/me`

Retrieves the current authenticated user's information.

**Query Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `full_info` | boolean | `false` | If `true`, includes Keycloak data (username, email, first name, last name) |

**Response (basic):**

```json
{
  "id": "uuid",
  "display_name": "string",
  "profile_picture": "string | null",
  "status": "string",
  "sub": "string"
}
```

**Response (full_info=true):**

```json
{
  "id": "uuid",
  "display_name": "string",
  "profile_picture": "string | null",
  "status": "string",
  "sub": "string",
  "username": "string",
  "email": "string",
  "first_name": "string",
  "last_name": "string"
}
```

---

### PUT `/users/me`

Updates the current authenticated user's profile.

**Request Body:**

```json
{
  "display_name": "string (optional)",
  "profile_picture": "string | null (optional)",
  "status": "string (optional)"
}
```

**Response:**

```json
{
  "id": "uuid",
  "display_name": "string",
  "profile_picture": "string | null",
  "status": "string",
  "sub": "string"
}
```

---

### PUT `/users/me/keycloak`

Updates the current authenticated user's Keycloak information.

**Request Body:**

```json
{
  "username": "string (optional)",
  "email": "string (optional)",
  "first_name": "string (optional)",
  "last_name": "string (optional)"
}
```

**Response:** `200 OK` (no body)

---

### GET `/users/me/settings`

Retrieves the current authenticated user's parameters (theme, language).

**Response:**

```json
{
  "id": "uuid",
  "user_id": "uuid",
  "theme": "string | null",
  "lang": "string | null",
  "created_at": "2025-01-01T00:00:00Z",
  "updated_at": "2025-01-01T00:00:00Z"
}
```

---

### PUT `/users/me/settings`

Updates the current authenticated user's parameters.

**Request Body:**

```json
{
  "theme": "string (optional)",
  "lang": "string (optional)"
}
```

**Response:**

```json
{
  "id": "uuid",
  "user_id": "uuid",
  "theme": "string | null",
  "lang": "string | null",
  "created_at": "2025-01-01T00:00:00Z",
  "updated_at": "2025-01-01T00:00:00Z"
}
```

---

### GET `/users/:user_id`

Retrieves a specific user's information by their ID.

**Path Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `user_id` | UUID | The user's ID |

**Response:**

```json
{
  "id": "uuid",
  "display_name": "string",
  "profile_picture": "string | null",
  "status": "string",
  "sub": "string"
}
```

---

## Status Codes

| Code | Description |
|------|-------------|
| `200` | Success |
| `400` | Bad request |
| `401` | Unauthorized (missing or invalid JWT) |
| `404` | Resource not found |
| `500` | Server error |

## Error Format

```json
{
  "error": "Error description"
}
```

---

## cURL Examples

### Get current user

```bash
curl -X GET http://localhost:3000/users/me \
  -H "Authorization: Bearer <token>"
```

### Get current user with full info

```bash
curl -X GET "http://localhost:3000/users/me?full_info=true" \
  -H "Authorization: Bearer <token>"
```

### Update profile

```bash
curl -X PUT http://localhost:3000/users/me \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"display_name": "new_display_name", "status": "online"}'
```

### Update parameters

```bash
curl -X PUT http://localhost:3000/users/me/settings \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"theme": "dark", "lang": "en"}'
```

### Get another user

```bash
curl -X GET http://localhost:3000/users/550e8400-e29b-41d4-a716-446655440000 \
  -H "Authorization: Bearer <token>"
```
