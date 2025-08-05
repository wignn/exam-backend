# Exam API Documentation

## Overview
Exam management endpoints for creating, updating, retrieving, and managing exam assignments. Teacher role required for most operations.

## Endpoints

### 1. Get Exams (Paginated)
Retrieve list of exams with pagination support.

**Endpoint:** `GET /api/v1/exams`

**Headers:**
```http
Authorization: Bearer <access-token>
```

**Query Parameters:**
- `limit`: Number of items per page (default: 20, max: 100)
- `skip`: Number of items to skip (default: 0)
- `page`: Page number (alternative to skip)

**Examples:**
```http
GET /api/v1/exams
GET /api/v1/exams?limit=10&skip=20
GET /api/v1/exams?limit=5&page=3
```

**Response (200 OK):**
```json
{
  "message": "Exams retrieved successfully",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "title": "Midterm Exam - Data Structures",
      "description": "Comprehensive exam covering arrays, linked lists, trees, and graphs",
      "created_by": "123e4567-e89b-12d3-a456-426614174000",
      "duration_minutes": 120,
      "start_time": "2025-08-10T09:00:00Z",
      "end_time": "2025-08-10T11:00:00Z",
      "is_active": true
    }
  ],
  "pagination": {
    "limit": 20,
    "skip": 0,
    "page": 1
  }
}
```

---

### 2. Get Exam Detail
Retrieve detailed information about a specific exam.

**Endpoint:** `GET /api/v1/exams/{exam_id}`

**Headers:**
```http
Authorization: Bearer <access-token>
```

**Path Parameters:**
- `exam_id`: UUID of the exam

**Response (200 OK):**
```json
{
  "message": "Exam retrieved successfully",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "Midterm Exam - Data Structures",
    "description": "Comprehensive exam covering arrays, linked lists, trees, and graphs",
    "created_by": "123e4567-e89b-12d3-a456-426614174000",
    "duration_minutes": 120,
    "start_time": "2025-08-10T09:00:00Z",
    "end_time": "2025-08-10T11:00:00Z",
    "is_active": true
  }
}
```

**Error Responses:**
```json
// 404 Not Found - Exam not found
{
  "error": "Exam not found"
}
```

---

### 3. Create Exam
Create a new exam. **Teacher role required.**

**Endpoint:** `POST /api/v1/exams`

**Headers:**
```http
Authorization: Bearer <access-token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "title": "Final Exam - Algorithms",
  "description": "Final examination covering sorting, searching, and graph algorithms",
  "duration_minutes": 180,
  "start_time": "2025-08-15T09:00:00Z",
  "end_time": "2025-08-15T12:00:00Z"
}
```

**Request Validation:**
- `title`: Required, minimum 1 character
- `description`: Required, minimum 1 character
- `duration_minutes`: Required, minimum 1 minute
- `start_time`: Required, valid ISO 8601 datetime
- `end_time`: Required, valid ISO 8601 datetime

**Response (201 Created):**
```json
{
  "message": "Exam created successfully",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "Final Exam - Algorithms",
    "description": "Final examination covering sorting, searching, and graph algorithms",
    "created_by": "123e4567-e89b-12d3-a456-426614174000",
    "duration_minutes": 180,
    "start_time": "2025-08-15T09:00:00Z",
    "end_time": "2025-08-15T12:00:00Z",
    "is_active": true
  }
}
```

**Error Responses:**
```json
// 400 Bad Request - Validation Error
{
  "error": "Validation error: title must not be empty"
}

// 403 Forbidden - Insufficient permissions
{
  "error": "Teacher role required"
}
```

---

### 4. Update Exam
Update an existing exam. **Teacher role required.**

**Endpoint:** `PUT /api/v1/exams/{exam_id}`

**Headers:**
```http
Authorization: Bearer <access-token>
Content-Type: application/json
```

**Path Parameters:**
- `exam_id`: UUID of the exam to update

**Request Body:**
```json
{
  "title": "Updated Final Exam - Algorithms",
  "description": "Updated final examination covering advanced algorithms",
  "duration_minutes": 150,
  "start_time": "2025-08-15T10:00:00Z",
  "end_time": "2025-08-15T12:30:00Z",
  "is_active": false
}
```

**Request Validation:**
- `title`: Required, minimum 1 character
- `description`: Required, minimum 1 character
- `duration_minutes`: Required, minimum 1 minute
- `start_time`: Required, valid ISO 8601 datetime
- `end_time`: Required, valid ISO 8601 datetime
- `is_active`: Required, boolean

**Response (200 OK):**
```json
{
  "message": "Exam updated successfully",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "Updated Final Exam - Algorithms",
    "description": "Updated final examination covering advanced algorithms",
    "created_by": "123e4567-e89b-12d3-a456-426614174000",
    "duration_minutes": 150,
    "start_time": "2025-08-15T10:00:00Z",
    "end_time": "2025-08-15T12:30:00Z",
    "is_active": false
  }
}
```

---

### 5. Delete Exam
Delete an existing exam. **Teacher role required.**

**Endpoint:** `DELETE /api/v1/exams/{exam_id}`

**Headers:**
```http
Authorization: Bearer <access-token>
```

**Path Parameters:**
- `exam_id`: UUID of the exam to delete

**Response (200 OK):**
```json
{
  "message": "Exam deleted successfully"
}
```

**Error Responses:**
```json
// 403 Forbidden - Insufficient permissions
{
  "error": "Teacher role required"
}

// 404 Not Found - Exam not found
{
  "error": "Exam not found"
}
```

---

### 6. Assign Exam to Class
Assign an exam to a specific class. **Teacher role required.**

**Endpoint:** `POST /api/v1/exams/assignments`

**Headers:**
```http
Authorization: Bearer <access-token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "exam_id": "550e8400-e29b-41d4-a716-446655440000",
  "class_id": "123e4567-e89b-12d3-a456-426614174000"
}
```

**Response (201 Created):**
```json
{
  "message": "Exam assigned to class successfully",
  "data": {
    "exam_id": "550e8400-e29b-41d4-a716-446655440000",
    "class_id": "123e4567-e89b-12d3-a456-426614174000"
  }
}
```

**Error Responses:**
```json
// 403 Forbidden - Insufficient permissions
{
  "error": "Teacher role required"
}

// 404 Not Found - Exam or class not found
{
  "error": "Exam or class not found"
}

// 409 Conflict - Assignment already exists
{
  "error": "Exam is already assigned to this class"
}
```

---

### 7. Unassign Exam from Class
Remove exam assignment from a specific class. **Teacher role required.**

**Endpoint:** `DELETE /api/v1/exams/assignments`

**Headers:**
```http
Authorization: Bearer <access-token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "exam_id": "550e8400-e29b-41d4-a716-446655440000",
  "class_id": "123e4567-e89b-12d3-a456-426614174000"
}
```

**Response (200 OK):**
```json
{
  "message": "Exam unassigned from class successfully"
}
```

**Error Responses:**
```json
// 403 Forbidden - Insufficient permissions
{
  "error": "Teacher role required"
}

// 404 Not Found - Assignment not found
{
  "error": "Exam assignment not found"
}
```

---

## Authentication & Authorization

### Required Headers
```http
Authorization: Bearer <access-token>
```

### Role Requirements
- **Get Exams**: Any authenticated user
- **Get Exam Detail**: Any authenticated user
- **Create Exam**: Teacher role
- **Update Exam**: Teacher role  
- **Delete Exam**: Teacher role
- **Assign/Unassign Exam**: Teacher role

## Data Types
- All timestamps use ISO 8601 format (UTC)
- All IDs are UUIDs
- Duration is specified in minutes
- Exam status is controlled by `is_active` boolean field
