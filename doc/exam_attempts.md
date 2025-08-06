# Exam Attempts API Documentation

## Overview
The Exam Attempts API manages student exam attempts, including starting exams, submitting answers, retrieving attempt history, and tracking exam progress. It supports automatic scoring for multiple choice and true/false questions, with manual grading capabilities for essay questions.

## Authentication
All endpoints require authentication via Bearer token in the Authorization header.

## Endpoints

### 1. Start Exam Attempt
Start a new exam attempt for the authenticated user.

**Endpoint:** `POST /api/v1/exam-attempts/start`

**Headers:**
```
Authorization: Bearer <access_token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "exam_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Request Validation:**
- `exam_id`: Required, valid UUID format

**Business Rules:**
- Exam must exist and be active
- Exam must be within the scheduled time window (start_time <= now <= end_time)
- User must have access to the exam through class membership
- User can only have one attempt per exam
- Exam attempt is automatically started with current timestamp

**Response (200 OK):**
```json
{
  "message": "Exam attempt started successfully",
  "data": {
    "id": "660e8400-e29b-41d4-a716-446655440001",
    "user_id": "550e8400-e29b-41d4-a716-446655440000",
    "exam_id": "550e8400-e29b-41d4-a716-446655440000",
    "started_at": "2025-08-05T10:30:00Z",
    "submitted_at": null,
    "score_total": null,
    "status": "in_progress"
  }
}
```

**Error Responses:**
- `404 Not Found`: Exam not found or not active
- `403 Forbidden`: User doesn't have access to this exam
- `409 Conflict`: User already has an attempt for this exam

---

### 2. Submit Exam Attempt
Submit answers for an exam attempt with automatic scoring.

**Endpoint:** `POST /api/v1/exam-attempts/submit`

**Headers:**
```
Authorization: Bearer <access_token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "attempt_id": "660e8400-e29b-41d4-a716-446655440001",
  "answers": [
    {
      "question_id": "770e8400-e29b-41d4-a716-446655440002",
      "answer_text": "Option A"
    },
    {
      "question_id": "880e8400-e29b-41d4-a716-446655440003",
      "answer_text": "true"
    },
    {
      "question_id": "990e8400-e29b-41d4-a716-446655440004",
      "answer_text": "This is my essay answer..."
    }
  ]
}
```

**Request Validation:**
- `attempt_id`: Required, valid UUID format
- `answers`: Required array of answer submissions
- `question_id`: Required, valid UUID format
- `answer_text`: Required string

**Business Rules:**
- Attempt must belong to the authenticated user
- Attempt must not be already submitted
- Exam time must not have expired (based on exam end_time or duration_minutes from start)
- Automatic scoring for multiple choice and true/false questions
- Essay questions are not automatically scored (manual grading required)
- Total score is calculated and stored

**Response (200 OK):**
```json
{
  "message": "Exam attempt submitted successfully",
  "data": {
    "id": "660e8400-e29b-41d4-a716-446655440001",
    "user_id": "550e8400-e29b-41d4-a716-446655440000",
    "exam_id": "550e8400-e29b-41d4-a716-446655440000",
    "started_at": "2025-08-05T10:30:00Z",
    "submitted_at": "2025-08-05T11:15:00Z",
    "score_total": 85,
    "status": "completed"
  }
}
```

**Error Responses:**
- `404 Not Found`: Exam attempt not found
- `400 Bad Request`: Exam already submitted or time expired

---

### 3. Get User Attempts
Retrieve all exam attempts for the authenticated user.

**Endpoint:** `GET /api/v1/exam-attempts/my-attempts`

**Headers:**
```
Authorization: Bearer <access_token>
```

**Response (200 OK):**
```json
{
  "message": "User exam attempts retrieved successfully",
  "data": [
    {
      "id": "660e8400-e29b-41d4-a716-446655440001",
      "user_id": "550e8400-e29b-41d4-a716-446655440000",
      "exam_id": "550e8400-e29b-41d4-a716-446655440000",
      "started_at": "2025-08-05T10:30:00Z",
      "submitted_at": "2025-08-05T11:15:00Z",
      "score_total": 85,
      "status": "completed"
    },
    {
      "id": "660e8400-e29b-41d4-a716-446655440002",
      "user_id": "550e8400-e29b-41d4-a716-446655440000",
      "exam_id": "550e8400-e29b-41d4-a716-446655440001",
      "started_at": "2025-08-04T14:00:00Z",
      "submitted_at": null,
      "score_total": null,
      "status": "in_progress"
    }
  ]
}
```

**Notes:**
- Results are ordered by started_at timestamp (most recent first)
- Status is automatically calculated based on submission state

---

### 4. Get Attempt Details with Answers
Retrieve a specific exam attempt with all submitted answers.

**Endpoint:** `GET /api/v1/exam-attempts/details/{attempt_id}`

**Headers:**
```
Authorization: Bearer <access_token>
```

**Path Parameters:**
- `attempt_id`: UUID of the exam attempt

**Response (200 OK):**
```json
{
  "message": "Exam attempt with answers retrieved successfully",
  "data": {
    "attempt": {
      "id": "660e8400-e29b-41d4-a716-446655440001",
      "user_id": "550e8400-e29b-41d4-a716-446655440000",
      "exam_id": "550e8400-e29b-41d4-a716-446655440000",
      "started_at": "2025-08-05T10:30:00Z",
      "submitted_at": "2025-08-05T11:15:00Z",
      "score_total": 85,
      "status": "completed"
    },
    "answers": [
      {
        "id": "770e8400-e29b-41d4-a716-446655440010",
        "attempt_id": "660e8400-e29b-41d4-a716-446655440001",
        "question_id": "770e8400-e29b-41d4-a716-446655440002",
        "answer_text": "Option A",
        "is_correct": true,
        "score_awarded": 10
      },
      {
        "id": "880e8400-e29b-41d4-a716-446655440011",
        "attempt_id": "660e8400-e29b-41d4-a716-446655440001",
        "question_id": "880e8400-e29b-41d4-a716-446655440003",
        "answer_text": "false",
        "is_correct": false,
        "score_awarded": 0
      },
      {
        "id": "990e8400-e29b-41d4-a716-446655440012",
        "attempt_id": "660e8400-e29b-41d4-a716-446655440001",
        "question_id": "990e8400-e29b-41d4-a716-446655440004",
        "answer_text": "This is my essay answer...",
        "is_correct": null,
        "score_awarded": null
      }
    ]
  }
}
```

**Error Responses:**
- `404 Not Found`: Exam attempt not found or doesn't belong to user

**Notes:**
- Answers are ordered by question_id
- Essay questions have `is_correct: null` and `score_awarded: null` until manually graded

---

### 5. Get Exam Attempts (Teachers Only)
Retrieve all attempts for a specific exam. Restricted to teachers and admins.

**Endpoint:** `GET /api/v1/exam-attempts/exam/{exam_id}`

**Headers:**
```
Authorization: Bearer <access_token>
```

**Path Parameters:**
- `exam_id`: UUID of the exam

**Authorization:** Requires `teacher` or `admin` role

**Response (200 OK):**
```json
{
  "message": "Exam attempts retrieved successfully",
  "data": [
    {
      "id": "660e8400-e29b-41d4-a716-446655440001",
      "user_id": "550e8400-e29b-41d4-a716-446655440000",
      "exam_id": "550e8400-e29b-41d4-a716-446655440000",
      "started_at": "2025-08-05T10:30:00Z",
      "submitted_at": "2025-08-05T11:15:00Z",
      "score_total": 85,
      "status": "completed"
    },
    {
      "id": "660e8400-e29b-41d4-a716-446655440002",
      "user_id": "550e8400-e29b-41d4-a716-446655440001",
      "exam_id": "550e8400-e29b-41d4-a716-446655440000",
      "started_at": "2025-08-05T11:00:00Z",
      "submitted_at": null,
      "score_total": null,
      "status": "in_progress"
    }
  ]
}
```

**Error Responses:**
- `403 Forbidden`: User doesn't have teacher or admin role

---

### 6. Get Active Attempt
Retrieve the current active (non-submitted) attempt for a user and exam.

**Endpoint:** `GET /api/v1/exam-attempts/active/{exam_id}`

**Headers:**
```
Authorization: Bearer <access_token>
```

**Path Parameters:**
- `exam_id`: UUID of the exam

**Response (200 OK) - With Active Attempt:**
```json
{
  "message": "Active exam attempt retrieved successfully",
  "data": {
    "id": "660e8400-e29b-41d4-a716-446655440001",
    "user_id": "550e8400-e29b-41d4-a716-446655440000",
    "exam_id": "550e8400-e29b-41d4-a716-446655440000",
    "started_at": "2025-08-05T10:30:00Z",
    "submitted_at": null,
    "score_total": null,
    "status": "in_progress"
  }
}
```

**Response (200 OK) - No Active Attempt:**
```json
{
  "message": "Active exam attempt retrieved successfully",
  "data": null
}
```

**Use Cases:**
- Check if user has an ongoing attempt before starting a new one
- Resume an existing attempt
- Display attempt status on exam interface

---

## Data Models

### ExamAttempt
```typescript
interface ExamAttempt {
  id: string;           // UUID
  user_id: string;      // UUID
  exam_id: string;      // UUID
  started_at: string;   // ISO 8601 DateTime (UTC)
  submitted_at: string | null; // ISO 8601 DateTime (UTC)
  score_total: number | null;  // Total score achieved
}
```

### ExamAttemptResponse
```typescript
interface ExamAttemptResponse extends ExamAttempt {
  status: "not_started" | "in_progress" | "completed";
}
```

### Answer
```typescript
interface Answer {
  id: string;           // UUID
  attempt_id: string;   // UUID
  question_id: string;  // UUID
  answer_text: string | null;
  is_correct: boolean | null;  // null for essay questions
  score_awarded: number | null; // null for ungraded essays
}
```

### StartExamAttemptRequest
```typescript
interface StartExamAttemptRequest {
  exam_id: string; // UUID
}
```

### SubmitExamAttemptRequest
```typescript
interface SubmitExamAttemptRequest {
  attempt_id: string; // UUID
  answers: AnswerSubmission[];
}

interface AnswerSubmission {
  question_id: string; // UUID
  answer_text: string;
}
```

---

## Status Calculation
The attempt status is automatically calculated based on the attempt state:
- `"not_started"`: started_at is null
- `"in_progress"`: started_at is not null, submitted_at is null
- `"completed"`: submitted_at is not null

---

## Scoring System

### Automatic Scoring
- **Multiple Choice Questions**: Exact match (case-insensitive) with correct_answer
- **True/False Questions**: Exact match (case-insensitive) with correct_answer
- **Essay Questions**: No automatic scoring, requires manual grading

### Score Calculation
- Each question has a configurable score value
- Total score is the sum of all awarded scores
- Incorrect answers receive 0 points
- Essay questions receive 0 points initially (until manually graded)

---

## Error Handling

### Common Error Responses

**400 Bad Request:**
```json
{
  "error": "Bad Request",
  "message": "Exam attempt already submitted"
}
```

**401 Unauthorized:**
```json
{
  "error": "Unauthorized",
  "message": "Invalid or missing authentication token"
}
```

**403 Forbidden:**
```json
{
  "error": "Forbidden",
  "message": "Insufficient permissions"
}
```

**404 Not Found:**
```json
{
  "error": "Not Found",
  "message": "Exam attempt not found"
}
```

**409 Conflict:**
```json
{
  "error": "Conflict",
  "message": "User already has an attempt for this exam"
}
```

---

## Usage Examples

### Starting an Exam
```bash
curl -X POST http://localhost:8080/api/v1/exam-attempts/start \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "exam_id": "550e8400-e29b-41d4-a716-446655440000"
  }'
```

### Submitting Answers
```bash
curl -X POST http://localhost:8080/api/v1/exam-attempts/submit \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "attempt_id": "660e8400-e29b-41d4-a716-446655440001",
    "answers": [
      {
        "question_id": "770e8400-e29b-41d4-a716-446655440002",
        "answer_text": "Option A"
      }
    ]
  }'
```

### Getting User Attempts
```bash
curl -X GET http://localhost:8080/api/v1/exam-attempts/my-attempts \
  -H "Authorization: Bearer <token>"
```

---

## Business Logic Notes

1. **Time Management**: Exam attempts are subject to both exam-level time limits (end_time) and duration-based limits (duration_minutes from start)

2. **Access Control**: Users can only access exams they're enrolled in through class memberships

3. **One Attempt Rule**: Each user can only have one attempt per exam

4. **Automatic Submission**: Consider implementing automatic submission when time expires (not currently implemented)

5. **Manual Grading**: Essay questions require manual intervention for scoring

6. **Data Integrity**: All operations use database transactions to ensure consistency
