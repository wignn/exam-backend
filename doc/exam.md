### Exam API Spec (Protected)

#### Get Exam List with Pagination
```http
GET /api/v1/exams
Authorization: Bearer <access-token>
```

Query Parameters (optional):
- `limit`: Number of items per page (default: 20)
- `skip`: Number of items to skip (default: 0)

Examples:
```http
GET /api/v1/exams?limit=10&skip=20
GET /api/v1/exams?limit=5
GET /api/v1/exams
```

Response:
```json
{
    "message": "Exams retrieved successfully",
    "exams": [
        {
            "id": "uuid",
            "title": "Exam Title",
            "description": "Exam Description",
            "created_by": "uuid",
            "duration_minutes": 120,
            "start_time": "2025-08-03T10:00:00Z",
            "end_time": "2025-08-03T12:00:00Z",
            "is_active": true
        }
    ]
}
```

#### Get Exam Detail
```http
GET /api/v1/exams/{exam_id}
Authorization: Bearer <access-token>
```

#### Create Exam (Teacher only)
```http
POST /api/v1/exams
Authorization: Bearer <access-token>
Content-Type: application/json

{
    "title": "Midterm Exam",
    "description": "Algorithm and Data Structure Midterm",
    "created_by": "teacher-uuid",
    "duration_minutes": 120,
    "start_time": "2025-08-10T10:00:00Z",
    "end_time": "2025-08-10T12:00:00Z"
}
```

#### Update Exam (Teacher only)
```http
PUT /api/v1/exams/{exam_id}
Authorization: Bearer <access-token>
Content-Type: application/json

{
    "title": "Updated Exam Title",
    "description": "Updated description",
    "duration_minutes": 150,
    "start_time": "2025-08-10T10:00:00Z",
    "end_time": "2025-08-10T12:30:00Z",
    "is_active": true
}
```

#### Delete Exam (Teacher only)
```http
DELETE /api/v1/exams/{exam_id}
Authorization: Bearer <access-token>
```

#### Assign Exam to Class (Teacher only)
```http
POST /api/v1/exams/assignments
Authorization: Bearer <access-token>
Content-Type: application/json

{
    "exam_id": "uuid",
    "class_id": "uuid"
}
```

#### Unassign Exam from Class (Teacher only)
```http
DELETE /api/v1/exams/assignments
Authorization: Bearer <access-token>
Content-Type: application/json

{
    "exam_id": "uuid",
    "class_id": "uuid"
}
```
