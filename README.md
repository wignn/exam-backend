# Exam Backend API

The exam application backend is built using Rust with the Axum framework. The application is designed to be easy to read, high-performance, and easy to develop.
## Fitur

### User Management
- ✅ Register user (admin, teacher, student)
- ✅ Login with email/password
- ✅ JWT access token And refresh token
- ✅ Update profile
- ✅ Change Password
- ✅ Verification email
- ✅ Role-based access control

### Security
- Password hashing with bcrypt
- JWT authentication with access and refresh token
- Middleware for protection endpoint
- Role-based authorization

## Tech

- **Framework**: Axum
- **Database**: PostgreSQL dengan SQLx
- **Authentication**: JWT (jsonwebtoken)
- **Password**: bcrypt
- **Validation**: validator
- **Error Handling**: thiserror, anyhow

## Setup

### Prerequisites
- Rust 1.70+
- PostgreSQL 13+

### Installation

1. Clone repository
```bash
git clone <repository-url>
cd exam-backend
```

2. Setup database
```bash
createdb exam_db
```

3. Copy environment file
```bash
cp .env.example .env
```

4. Update .env
```env
DATABASE_URL=postgresql://username:password@localhost/exam_db
JWT_SECRET=your-super-secret-jwt-key
```

5. Install SQLx CLI (for migration)
```bash
cargo install sqlx-cli
```

6. Run migrations
```bash
sqlx migrate run
```

7. Run Application
```bash
cargo run
```

Server running on `http://localhost:3000`

## API Endpoints


## Structure Project

```
src/
├── main.rs              # Entry point
├── config.rs            # Configuration management
├── database.rs          # Database connection
├── errors.rs            # Error handling
├── routes.rs            # Route definitions
├── handlers/            # HTTP handlers
│   ├── mod.rs
│   ├── auth.rs          # Authentication handlers
│   └── user.rs          # User handlers
├── middleware/          # Custom middleware
│   ├── mod.rs
│   └── auth.rs          # Authentication middleware
├── models/              # Data models
│   ├── mod.rs
│   └── user.rs          # User models
├── services/            # Business logic
│   ├── mod.rs
│   └── user.rs          # User service
└── utils/               # Utilities
    ├── mod.rs
    ├── jwt.rs           # JWT utilities
    └── password.rs      # Password utilities
```

## Development

### Adding New Features

1. **Models**: : Add structs in `src/models/`
2. **Services**: Add business logic in `src/services/`
3. **Handlers**: Add HTTP handlers in `src/handlers/`
4. **Routes**: Register routes in `src/main.rs`

### Database Migrations

Create new migration:
```bash
sqlx migrate add <migration_name>
```

Run migrations:
```bash
sqlx migrate run
```

### Testing

```bash
cargo test
```

## Konfigurasi Environment

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection URL | `postgresql://localhost/exam_db` |
| `JWT_SECRET` | JWT signing secret | `your-super-secret-jwt-key` |
| `JWT_ACCESS_EXPIRES_IN` | Access token expiry (seconds) | `900` (15 minutes) |
| `JWT_REFRESH_EXPIRES_IN` | Refresh token expiry (seconds) | `604800` (7 days) |
| `SERVER_HOST` | Server host | `0.0.0.0` |
| `SERVER_PORT` | Server port | `3000` |

## Performance

- Async/await untuk non-blocking I/O
- Connection pooling untuk database
- Efficient JSON serialization/deserialization
- Minimal allocations dengan zero-copy parsing

## Security Features

- Password hashing dengan salt
- JWT dengan expiry time
- CORS protection
- Input validation
- SQL injection protection via SQLx
- Role-based access control

## Next Steps

To add an exam management feature:

1. Add models for Exam, Question, Answer, etc.
2. Implement services for exam logic
3. Add handlers and routes 
4. Implement real-time features using WebSocket (optional)

## Contributing

1. Fork repository
2. Create feature branch
3. Commit changes
4. Push to branch
5. Create Pull Request