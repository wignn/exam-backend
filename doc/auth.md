
### Authentication API Spec

#### Register
```http
POST /api/v1/auth/register
Content-Type: application/json

{
  "name": "wign",
  "email": "wign@example.com",
  "password": "password123",
  "role": "admin"
}
```

#### Login
```http
POST /api/v1/auth/login
Content-Type: application/json

{
  "email": "wign@example.com",
  "password": "password123"
}
```

#### Refresh Token
```http
POST /api/v1/auth/refresh
Content-Type: application/json

{
  "refresh_token": "your-refresh-token"
}
```

#### Logout
```http
POST /api/v1/auth/logout
Authorization: Bearer <access-token>
```