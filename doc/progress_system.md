# Progress and Level System API Documentation

## Overview
The Progress and Level System tracks user learning progress, manages experience points, calculates levels, unlocks achievements, and provides gamification features to enhance user engagement.

## Features
- **User Progress Tracking**: Monitor completion of courses, exams, assignments, and quizzes
- **Experience Points System**: Earn XP based on performance and completion
- **Dynamic Level Calculation**: Automatic level progression based on total experience
- **Achievement System**: Unlock achievements based on various conditions
- **Leaderboard**: Compare progress with other users
- **Progress Analytics**: Detailed statistics and summaries

## Authentication
All endpoints require authentication via Bearer token in the Authorization header.

## Endpoints

### 1. Create Progress Entry
Manually create a new progress entry for a user.

**Endpoint:** `POST /api/v1/progress`

**Headers:**
```
Authorization: Bearer <access_token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "course_name": "Algorithm and Data Structure",
  "course_type": "course",
  "status": "started",
  "total_score": null,
  "max_score": null,
  "experience_points": 0
}
```

**Request Validation:**
- `course_name`: Required, minimum 1 character
- `course_type`: Required, one of: "course", "exam", "assignment", "quiz", "project"
- `status`: Required, one of: "started", "inprogress", "completed", "failed", "enrolled"

**Response (200 OK):**
```json
{
  "message": "Progress entry created successfully",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "user_id": "660e8400-e29b-41d4-a716-446655440001",
    "course_name": "Algorithm and Data Structure",
    "course_type": "course",
    "progress_percentage": null,
    "status": "started",
    "started_at": "2025-08-06T10:30:00Z",
    "completed_at": null,
    "total_score": null,
    "max_score": null,
    "level": 1,
    "experience_points": 0
  }
}
```

---

### 2. Update Progress
Update an existing progress entry.

**Endpoint:** `PUT /api/v1/progress/{progress_id}`

**Headers:**
```
Authorization: Bearer <access_token>
Content-Type: application/json
```

**Path Parameters:**
- `progress_id`: UUID of the progress entry

**Request Body:**
```json
{
  "progress_percentage": 92,
  "status": "completed",
  "total_score": 92,
  "completed_at": "2025-08-06T12:00:00Z",
  "experience_points": 150
}
```

**Response (200 OK):**
```json
{
  "message": "Progress updated successfully",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "user_id": "660e8400-e29b-41d4-a716-446655440001",
    "course_name": "Algorithm and Data Structure",
    "course_type": "course",
    "progress_percentage": 92,
    "status": "completed",
    "started_at": "2025-08-06T10:30:00Z",
    "completed_at": "2025-08-06T12:00:00Z",
    "total_score": 92,
    "max_score": 100,
    "level": 1,
    "experience_points": 200
  }
}
```

**Business Logic:**
- Completion bonus experience is automatically calculated
- Achievements are checked and unlocked if conditions are met
- User level is updated if experience threshold is reached

---

### 3. Get User Progress History
Retrieve the authenticated user's progress history.

**Endpoint:** `GET /api/v1/progress/my-progress`

**Headers:**
```
Authorization: Bearer <access_token>
```

**Query Parameters:**
- `limit` (optional): Maximum number of entries to return (default: 50, max: 50)

**Response (200 OK):**
```json
{
  "message": "User progress retrieved successfully",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "user_id": "660e8400-e29b-41d4-a716-446655440001",
      "course_name": "Algorithm and Data Structure",
      "course_type": "course",
      "progress_percentage": 92,
      "status": "completed",
      "started_at": "2025-08-06T10:30:00Z",
      "completed_at": "2025-08-06T12:00:00Z",
      "total_score": 92,
      "max_score": 100,
      "level": 2,
      "experience_points": 200
    },
    {
      "id": "550e8400-e29b-41d4-a716-446655440001",
      "user_id": "660e8400-e29b-41d4-a716-446655440001",
      "course_name": "JavaScript Fundamentals",
      "course_type": "course",
      "progress_percentage": 85,
      "status": "completed",
      "started_at": "2025-08-05T14:00:00Z",
      "completed_at": "2025-08-05T16:30:00Z",
      "total_score": 85,
      "max_score": 100,
      "level": 1,
      "experience_points": 150
    }
  ]
}
```

---

### 4. Get User Level
Retrieve the authenticated user's current level and experience information.

**Endpoint:** `GET /api/v1/progress/my-level`

**Headers:**
```
Authorization: Bearer <access_token>
```

**Response (200 OK):**
```json
{
  "message": "User level retrieved successfully",
  "data": {
    "id": "770e8400-e29b-41d4-a716-446655440000",
    "user_id": "660e8400-e29b-41d4-a716-446655440001",
    "current_level": 3,
    "total_experience": 650,
    "experience_to_next_level": 350,
    "level_title": "Intermediate",
    "achievements": [
      "First Steps",
      "High Achiever",
      "Dedicated Learner"
    ],
    "updated_at": "2025-08-06T12:00:00Z"
  }
}
```

---

### 5. Get Progress Summary
Get a comprehensive dashboard summary of user's progress.

**Endpoint:** `GET /api/v1/progress/summary`

**Headers:**
```
Authorization: Bearer <access_token>
```

**Response (200 OK):**
```json
{
  "message": "Progress summary retrieved successfully",
  "data": {
    "user_level": {
      "id": "770e8400-e29b-41d4-a716-446655440000",
      "user_id": "660e8400-e29b-41d4-a716-446655440001",
      "current_level": 3,
      "total_experience": 650,
      "experience_to_next_level": 350,
      "level_title": "Intermediate",
      "achievements": [
        "First Steps",
        "High Achiever"
      ],
      "updated_at": "2025-08-06T12:00:00Z"
    },
    "recent_activities": [
      {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "course_name": "Midterm Exam",
        "course_type": "exam",
        "progress_percentage": 88,
        "status": "completed",
        "started_at": "2025-08-06T09:00:00Z",
        "completed_at": "2025-08-06T11:00:00Z",
        "experience_points": 175
      }
    ],
    "completed_courses": 5,
    "courses_in_progress": 2,
    "total_experience_earned": 650,
    "achievements_unlocked": 2
  }
}
```

---

### 6. Get User Progress by ID (Teachers Only)
Retrieve another user's progress history. Restricted to teachers and admins.

**Endpoint:** `GET /api/v1/progress/user/{user_id}/progress`

**Headers:**
```
Authorization: Bearer <access_token>
```

**Path Parameters:**
- `user_id`: UUID of the target user

**Authorization:** Requires `teacher` or `admin` role

**Query Parameters:**
- `limit` (optional): Maximum number of entries to return

**Response (200 OK):**
```json
{
  "message": "User progress retrieved successfully",
  "data": [
    // Array of progress entries (same format as my-progress)
  ]
}
```

---

### 7. Get User Level by ID (Teachers Only)
Retrieve another user's level information.

**Endpoint:** `GET /api/v1/progress/user/{user_id}/level`

**Headers:**
```
Authorization: Bearer <access_token>
```

**Path Parameters:**
- `user_id`: UUID of the target user

**Authorization:** Requires `teacher` or `admin` role

**Response:** Same format as `my-level` endpoint

---

### 8. Get Leaderboard
Retrieve the top users by level and experience.

**Endpoint:** `GET /api/v1/progress/leaderboard`

**Query Parameters:**
- `limit` (optional): Number of top users to return (default: 10, max: 50)

**Response (200 OK):**
```json
{
  "message": "Leaderboard retrieved successfully",
  "data": [
    {
      "user_id": "660e8400-e29b-41d4-a716-446655440001",
      "name": "Alice Johnson",
      "level": 8,
      "total_experience": 2850,
      "level_title": "Advanced"
    },
    {
      "user_id": "660e8400-e29b-41d4-a716-446655440002",
      "name": "Bob Smith",
      "level": 6,
      "total_experience": 1750,
      "level_title": "Advanced"
    }
  ]
}
```

---

## Data Models

### CourseType Enum
```typescript
type CourseType = "course" | "exam" | "assignment" | "quiz" | "project";
```

### ProgressStatus Enum
```typescript
type ProgressStatus = "started" | "inprogress" | "completed" | "failed" | "enrolled";
```

### UserProgress
```typescript
interface UserProgress {
  id: string;
  user_id: string;
  course_name: string;
  course_type: CourseType;
  progress_percentage: number | null; // 0-100
  status: ProgressStatus;
  started_at: string; // ISO 8601 DateTime
  completed_at: string | null;
  total_score: number | null;
  max_score: number | null;
  level: number;
  experience_points: number;
}
```

### UserLevel
```typescript
interface UserLevel {
  id: string;
  user_id: string;
  current_level: number;
  total_experience: number;
  experience_to_next_level: number;
  level_title: string;
  achievements: string[];
  updated_at: string;
}
```

---

## Level System

### Level Calculation
Experience requirements for each level:

| Level | Experience Required | Level Title |
|-------|-------------------|-------------|
| 1     | 0-99              | Beginner    |
| 2     | 100-299           | Beginner    |
| 3     | 300-599           | Intermediate|
| 4     | 600-999           | Intermediate|
| 5     | 1000-1499         | Intermediate|
| 6     | 1500-2099         | Advanced    |
| 7     | 2100-2799         | Advanced    |
| 8     | 2800-3599         | Advanced    |
| 9     | 3600-4499         | Expert      |
| 10+   | 4500+             | Master      |

### Experience Points System

**Exam Completion:**
- 90-100%: 200 XP
- 80-89%: 150 XP  
- 70-79%: 100 XP
- 60-69%: 75 XP
- Below 60%: 50 XP

**Course/Assignment Completion:**
- Base: 50 XP + score bonus

**Achievement Unlocks:**
- First Steps: +100 XP
- High Achiever: +200 XP
- Dedicated Learner: +300 XP
- Experience Master: +500 XP
- Speed Runner: +250 XP

---

## Achievement System

### Available Achievements

1. **First Steps** 🎯
   - *Description*: Complete your first exam
   - *Condition*: Complete 1 exam
   - *Reward*: 100 XP

2. **High Achiever** 🏆
   - *Description*: Score above 90% in an exam
   - *Condition*: Score ≥ 90%
   - *Reward*: 200 XP

3. **Dedicated Learner** 📚
   - *Description*: Study for 7 consecutive days
   - *Condition*: 7 consecutive active days
   - *Reward*: 300 XP

4. **Experience Master** 💎
   - *Description*: Reach 1000 total experience points
   - *Condition*: Total XP ≥ 1000
   - *Reward*: 500 XP

5. **Speed Runner** ⚡
   - *Description*: Complete an exam in under 30 minutes
   - *Condition*: Exam duration < 30 minutes
   - *Reward*: 250 XP

---

## Automatic Integration

### Exam Attempts Integration
The progress system is automatically integrated with exam attempts:

1. **Starting an Exam**: Creates a progress entry with status "started"
2. **Completing an Exam**: Updates progress with:
   - Final score and percentage
   - Status changed to "completed"
   - Experience points awarded based on performance
   - Achievement checks performed
   - Level updated if experience threshold reached

---

## Database Schema

### Required Tables

```sql
-- Create user_progress table
CREATE TABLE user_progress (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    course_name VARCHAR(255) NOT NULL,
    course_type course_type NOT NULL,
    progress_percentage INTEGER CHECK (progress_percentage >= 0 AND progress_percentage <= 100),
    status progress_status NOT NULL DEFAULT 'started',
    started_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE,
    total_score INTEGER,
    max_score INTEGER,
    level INTEGER NOT NULL DEFAULT 1,
    experience_points INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create user_levels table
CREATE TABLE user_levels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE REFERENCES users(id),
    current_level INTEGER NOT NULL DEFAULT 1,
    total_experience INTEGER NOT NULL DEFAULT 0,
    experience_to_next_level INTEGER NOT NULL DEFAULT 100,
    level_title VARCHAR(50) NOT NULL DEFAULT 'Beginner',
    achievements JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
```

---

## Usage Examples

### Creating Manual Progress Entry
```bash
curl -X POST http://localhost:8080/api/v1/progress \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "course_name": "React Advanced Patterns",
    "course_type": "course",
    "status": "started",
    "experience_points": 0
  }'
```

### Updating Progress
```bash
curl -X PUT http://localhost:8080/api/v1/progress/{progress_id} \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "progress_percentage": 100,
    "status": "completed",
    "total_score": 95,
    "experience_points": 180
  }'
```

### Getting Progress Summary
```bash
curl -X GET http://localhost:8080/api/v1/progress/summary \
  -H "Authorization: Bearer <token>"
```

---

## Error Handling

**400 Bad Request:**
```json
{
  "error": "Bad Request",
  "message": "Invalid progress percentage. Must be between 0 and 100"
}
```

**403 Forbidden:**
```json
{
  "error": "Forbidden",
  "message": "Insufficient permissions to access this user's progress"
}
```

**404 Not Found:**
```json
{
  "error": "Not Found",
  "message": "Progress entry not found"
}
```
