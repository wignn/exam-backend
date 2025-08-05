### Class API Spec (Protected)

#### Get Class List with Pagination
```http
GET /api/v1/classes
Authorization: Bearer <access-token>
```

Query Parameters (optional):
- `limit`: Number of items per page (default: 20)
- `skip`: Number of items to skip (default: 0)

Examples:
```http
GET /api/v1/classes?limit=10&skip=20
GET /api/v1/classes?limit=5
GET /api/v1/classes
```

Response:
```json
{
    "message": "Classes retrieved successfully",
    "class": [
        {
            "id": "uuid",
            "name": "Class Name",
            "created_by": "uuid",
            "created_at": "2025-08-03T10:00:00Z"
        }
    ]
}
```

#### Get Class Detail
```http
GET /api/v1/classes/{class_id}
Authorization: Bearer <access-token>
```

#### Create Class (Teacher only)
```http
POST /api/v1/classes
Authorization: Bearer <access-token>
Content-Type: application/json

{
    "name": "algoritma"
}
```

#### Update Class
```http
PUT /api/v1/classes/{class_id}
Authorization: Bearer <access-token>
Content-Type: application/json

{
    "name": "new name"
}
```

#### Delete Class
```http
DELETE /api/v1/classes/{class_id}
Authorization: Bearer <access-token>
```

#### Add Class Member
```http
POST /api/v1/classes/members
Authorization: Bearer <access-token>
Content-Type: application/json

{
    "user_id": "uuid",
    "class_id": "uuid"
}
```

#### Remove Class Member
```http
DELETE /api/v1/classes/members
Authorization: Bearer <access-token>
Content-Type: application/json

{
    "user_id": "uuid",
    "class_id": "uuid"
}
```

#### Get Class Members
```http
GET /api/v1/classes/{class_id}/members
Authorization: Bearer <access-token>
```