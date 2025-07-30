### Class API Spec (Protected)

#### Get Class List
```http
GET /api/v1/classes
Authotization: Bearer <access-token>
```

#### Get Class Detail
```http
GET /api/v1/classes/{class_id}
Authotization: Bearer <access-token>
```

#### Create Class
```http
POST /api/v1/classes
Authotization: Bearer <access-token>
Content-Type: application/json

{
    "name":"algoritma",
    "created_by":"f1cf9513-6c0d-45ee-a720-ca038bcffc02"
}
```

#### Update Class
```http
PUT /api/v1/classes/{class_id}
Authotization: Bearer <access-token>
{
    "name":"new name
}
```



#### Delete Class
```http
DELETE /api/v1/classes/{class_id}
Authotization: Bearer <access-token>
```