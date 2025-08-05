
# User API Documentation

## Overview
User profile management endpoints for authenticated users. All endpoints require valid JWT access token.

## Endpoints

### 1. Get Profile
Retrieve current user's profile information.

**Endpoint:** `GET /api/v1/users/profile`

**Headers:**
```http
Authorization: Bearer <access-token>
```

**Request Body:** None

**Response (200 OK):**
```json
{
  "message": "Profile retrieved successfully",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "John Doe",
    "email": "john.doe@example.com",
    "is_active": true,
    "is_verified": false,
    "role": "student",
    "created_at": "2025-08-05T10:30:00Z",
    "updated_at": "2025-08-05T10:30:00Z"
  }
}
```

**Error Responses:**
```json
// 401 Unauthorized - Invalid or missing token
{
  "error": "Invalid or missing authorization token"
}

// 404 Not Found - User not found
{
  "error": "User not found"
}
```

---

### 2. Update Profile
Update current user's profile information.

**Endpoint:** `POST /api/v1/users/profile`

**Headers:**
```http
Authorization: Bearer <access-token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "name": "John Smith"
}
```

**Request Validation:**
- `name`: Optional, 2-100 characters when provided

**Response (200 OK):**
```json
{
  "message": "Profile updated successfully",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "John Smith",
    "email": "john.doe@example.com",
    "is_active": true,
    "is_verified": false,
    "role": "student",
    "created_at": "2025-08-05T10:30:00Z",
    "updated_at": "2025-08-05T11:45:00Z"
  }
}
```

**Error Responses:**
```json
// 400 Bad Request - Validation Error
{
  "error": "Validation error: name must be between 2 and 100 characters"
}

// 401 Unauthorized - Invalid or missing token
{
  "error": "Invalid or missing authorization token"
}
```

---

### 3. Change Password
Change current user's password.

**Endpoint:** `POST /api/v1/users/change-password`

**Headers:**
```http
Authorization: Bearer <access-token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "current_password": "oldpassword123",
  "new_password": "newpassword456"
}
```

**Request Validation:**
- `current_password`: Required, non-empty string
- `new_password`: Required, minimum 8 characters

**Response (200 OK):**
```json
{
  "message": "Password changed successfully"
}
```

**Error Responses:**
```json
// 400 Bad Request - Validation Error
{
  "error": "Validation error: new_password must be at least 8 characters"
}

// 400 Bad Request - Wrong current password
{
  "error": "Current password is incorrect"
}

// 401 Unauthorized - Invalid or missing token
{
  "error": "Invalid or missing authorization token"
}
```

---

### 4. Verify Email
Verify user's email address using verification token.

**Endpoint:** `POST /api/v1/users/verify-email`

**Headers:**
```http
Authorization: Bearer <access-token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "token": "email-verification-token"
}
```

**Response (200 OK):**
```json
{
  "message": "Email verified successfully",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "John Doe",
    "email": "john.doe@example.com",
    "is_active": true,
    "is_verified": true,
    "role": "student",
    "created_at": "2025-08-05T10:30:00Z",
    "updated_at": "2025-08-05T12:00:00Z"
  }
}
```

**Error Responses:**
```json
// 400 Bad Request - Invalid verification token
{
  "error": "Invalid or expired verification token"
}

// 401 Unauthorized - Invalid or missing token
{
  "error": "Invalid or missing authorization token"
}

// 409 Conflict - Email already verified
{
  "error": "Email is already verified"
}
```

---

## Authentication
All endpoints require a valid JWT access token in the Authorization header:

```http
Authorization: Bearer <access-token>
```

## User Roles
- **admin**: Full system access
- **teacher**: Can create/manage exams and classes
- **student**: Can take exams and view results
