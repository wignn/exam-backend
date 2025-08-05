
# Authentication API Documentation

## Overview
Authentication system supporting user registration, login, token refresh, and logout functionality with role-based access control.

## Endpoints

### 1. Register
Create a new user account.

**Endpoint:** `POST /api/v1/auth/register`

**Headers:**
```
Content-Type: application/json
```

**Request Body:**
```json
{
  "name": "John Doe",
  "email": "john.doe@example.com", 
  "password": "password123",
  "role": "student"
}
```

**Request Validation:**
- `name`: Required, 2-100 characters
- `email`: Required, valid email format
- `password`: Required, minimum 8 characters
- `role`: Required, one of: "admin", "teacher", "student"

**Response (201 Created):**
```json
{
  "message": "User registered successfully",
  "data": {
    "user": {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "John Doe",
      "email": "john.doe@example.com",
      "is_active": true,
      "is_verified": false,
      "role": "student",
      "created_at": "2025-08-05T10:30:00Z",
      "updated_at": "2025-08-05T10:30:00Z"
    },
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  }
}
```

**Error Responses:**
```json
// 400 Bad Request - Validation Error
{
  "error": "Validation error: email must be a valid email"
}

// 409 Conflict - Email already exists
{
  "error": "Email already exists"
}
```

---

### 2. Login
Authenticate user and get access tokens.

**Endpoint:** `POST /api/v1/auth/login`

**Headers:**
```
Content-Type: application/json
```

**Request Body:**
```json
{
  "email": "john.doe@example.com",
  "password": "password123"
}
```

**Request Validation:**
- `email`: Required, valid email format
- `password`: Required, non-empty string

**Response (200 OK):**
```json
{
  "message": "Login successful",
  "data": {
    "user": {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "John Doe",
      "email": "john.doe@example.com",
      "is_active": true,
      "is_verified": false,
      "role": "student",
      "created_at": "2025-08-05T10:30:00Z",
      "updated_at": "2025-08-05T10:30:00Z"
    },
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  }
}
```

**Error Responses:**
```json
// 400 Bad Request - Validation Error
{
  "error": "Validation error: email must be a valid email"
}

// 401 Unauthorized - Invalid credentials
{
  "error": "Invalid email or password"
}

// 403 Forbidden - Account not active
{
  "error": "Account is not active"
}
```

---

### 3. Refresh Token
Get new access token using refresh token.

**Endpoint:** `POST /api/v1/auth/refresh`

**Headers:**
```
Content-Type: application/json
```

**Request Body:**
```json
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Response (200 OK):**
```json
{
  "message": "Token refreshed successfully",
  "data": {
    "user": {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "John Doe",
      "email": "john.doe@example.com",
      "is_active": true,
      "is_verified": false,
      "role": "student",
      "created_at": "2025-08-05T10:30:00Z",
      "updated_at": "2025-08-05T10:30:00Z"
    },
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  }
}
```

**Error Responses:**
```json
// 401 Unauthorized - Invalid or expired refresh token
{
  "error": "Invalid or expired refresh token"
}
```

---

### 4. Logout
Logout user (invalidate tokens).

**Endpoint:** `POST /api/v1/auth/logout`

**Headers:**
```
Authorization: Bearer <access-token>
Content-Type: application/json
```

**Request Body:** None

**Response (200 OK):**
```json
{
  "message": "Logout successful"
}
```

**Error Responses:**
```json
// 401 Unauthorized - Invalid or missing token
{
  "error": "Invalid or missing authorization token"
}
```

---

## Authentication Flow

1. **Registration/Login**: User provides credentials and receives both access and refresh tokens
2. **API Access**: Use access token in Authorization header for protected endpoints
3. **Token Refresh**: When access token expires, use refresh token to get new tokens
4. **Logout**: Invalidate tokens (client should discard stored tokens)

## Token Usage

**Protected endpoints require Authorization header:**
```
Authorization: Bearer <access-token>
```

**Token Expiration:**
- Access Token: Short-lived (typically 15-60 minutes)
- Refresh Token: Long-lived (typically 7-30 days)

## User Roles

- **admin**: Full system access, can manage all resources
- **teacher**: Can create/manage exams and classes, view student results
- **student**: Can take exams, view own results