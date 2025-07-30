
### User Profile API Spec (Protected) 

#### Get Profile
```http
GET /api/v1/users/profile
Authorization: Bearer <access-token>
```

#### Update Profile
```http
POST /api/v1/users/profile
Authorization: Bearer <access-token>
Content-Type: application/json

{
  "name": "wign"
}
```

#### Change Password
```http
POST /api/v1/users/change-password
Authorization: Bearer <access-token>
Content-Type: application/json

{
  "current_password": "oldpassword",
  "new_password": "newpassword123"
}
```

#### Verify Email
```http
POST /api/v1/users/verify-email
Authorization: Bearer <access-token>
Content-Type: application/json

{
  "token": "verification-token"
}
```
