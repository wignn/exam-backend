# Question API Documentation

## Overview
Question management endpoints for creating, updating, retrieving, and managing exam questions. Teacher role required for most operations.

## Endpoints

### 1. Get Questions by Exam
Retrieve all questions for a specific exam with pagination support.

**Endpoint:** `GET /api/v1/questions/exam/{exam_id}`

**Headers:**
```http
Authorization: Bearer <access-token>
```

**Path Parameters:**
- `exam_id`: UUID of the exam

**Query Parameters:**
- `limit`: Number of items per page (default: 20)
- `skip`: Number of items to skip (default: 0)
- `page`: Page number (alternative to skip)

**Response (200 OK):**
```json
{
  "message": "Questions retrieved successfully",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "exam_id": "123e4567-e89b-12d3-a456-426614174000",
      "question_text": "What is the time complexity of quicksort in the worst case?",
      "question_type": "multiple_choice",
      "correct_answer": "O(n²)",
      "score": 10,
      "choices": [
        {
          "id": "choice-uuid-1",
          "question_id": "550e8400-e29b-41d4-a716-446655440000",
          "choice_text": "O(n log n)",
          "is_correct": false,
          "created_at": "2025-08-05T10:30:00Z"
        },
        {
          "id": "choice-uuid-2",
          "question_id": "550e8400-e29b-41d4-a716-446655440000",
          "choice_text": "O(n²)",
          "is_correct": true,
          "created_at": "2025-08-05T10:30:00Z"
        }
      ]
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

### 2. Get Questions for Student
Retrieve questions for student view (without correct answers).

**Endpoint:** `GET /api/v1/questions/exam/{exam_id}/student`

**Headers:**
```http
Authorization: Bearer <access-token>
```

**Path Parameters:**
- `exam_id`: UUID of the exam

**Response (200 OK):**
```json
{
  "message": "Questions retrieved successfully",
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "exam_id": "123e4567-e89b-12d3-a456-426614174000",
      "question_text": "What is the time complexity of quicksort in the worst case?",
      "question_type": "multiple_choice",
      "score": 10,
      "choices": [
        {
          "id": "choice-uuid-1",
          "choice_text": "O(n log n)"
        },
        {
          "id": "choice-uuid-2",
          "choice_text": "O(n²)"
        }
      ]
    }
  ]
}
```

---

### 3. Get Question Detail
Retrieve detailed information about a specific question.

**Endpoint:** `GET /api/v1/questions/{question_id}`

**Headers:**
```http
Authorization: Bearer <access-token>
```

**Path Parameters:**
- `question_id`: UUID of the question

**Response (200 OK):**
```json
{
  "message": "Question retrieved successfully",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "exam_id": "123e4567-e89b-12d3-a456-426614174000",
    "question_text": "What is the time complexity of quicksort in the worst case?",
    "question_type": "multiple_choice",
    "correct_answer": "O(n²)",
    "score": 10,
    "choices": [
      {
        "id": "choice-uuid-1",
        "question_id": "550e8400-e29b-41d4-a716-446655440000",
        "choice_text": "O(n log n)",
        "is_correct": false,
        "created_at": "2025-08-05T10:30:00Z"
      },
      {
        "id": "choice-uuid-2",
        "question_id": "550e8400-e29b-41d4-a716-446655440000",
        "choice_text": "O(n²)",
        "is_correct": true,
        "created_at": "2025-08-05T10:30:00Z"
      }
    ]
  }
}
```

---

### 4. Create Question
Create a new question for an exam. **Teacher role required.**

**Endpoint:** `POST /api/v1/questions/{exam_id}`

**Headers:**
```http
Authorization: Bearer <access-token>
Content-Type: application/json
```

**Path Parameters:**
- `exam_id`: UUID of the exam

**Request Body:**
```json
{
  "question_text": "Explain the difference between stack and queue data structures.",
  "question_type": "essay",
  "correct_answer": "Stack follows LIFO principle while queue follows FIFO principle.",
  "score": 15,
  "choices": null
}
```

**Multiple Choice Example:**
```json
{
  "question_text": "What is the time complexity of binary search?",
  "question_type": "multiple_choice",
  "correct_answer": null,
  "score": 5,
  "choices": [
    {
      "choice_text": "O(n)",
      "is_correct": false
    },
    {
      "choice_text": "O(log n)",
      "is_correct": true
    },
    {
      "choice_text": "O(n log n)",
      "is_correct": false
    },
    {
      "choice_text": "O(1)",
      "is_correct": false
    }
  ]
}
```

**Request Validation:**
- `question_text`: Required, minimum 1 character
- `question_type`: Required, one of: "multiple_choice", "essay", "true_false"
- `correct_answer`: Optional for multiple choice, required for essay/true_false
- `score`: Required, minimum 1 point
- `choices`: Required for multiple_choice, optional for others

**Response (201 Created):**
```json
{
  "message": "Question created successfully",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "exam_id": "123e4567-e89b-12d3-a456-426614174000",
    "question_text": "What is the time complexity of binary search?",
    "question_type": "multiple_choice",
    "correct_answer": null,
    "score": 5,
    "choices": [
      {
        "id": "choice-uuid-1",
        "question_id": "550e8400-e29b-41d4-a716-446655440000",
        "choice_text": "O(n)",
        "is_correct": false,
        "created_at": "2025-08-05T10:30:00Z"
      }
    ]
  }
}
```

---

### 5. Update Question
Update an existing question. **Teacher role required.**

**Endpoint:** `PUT /api/v1/questions/{question_id}`

**Headers:**
```http
Authorization: Bearer <access-token>
Content-Type: application/json
```

**Path Parameters:**
- `question_id`: UUID of the question to update

**Request Body:**
```json
{
  "question_text": "Updated: What is the average time complexity of binary search?",
  "question_type": "multiple_choice",
  "correct_answer": null,
  "score": 8,
  "choices": [
    {
      "id": "existing-choice-uuid",
      "choice_text": "O(n)",
      "is_correct": false,
      "delete": false
    },
    {
      "choice_text": "O(log n)",
      "is_correct": true
    },
    {
      "id": "choice-to-delete-uuid",
      "choice_text": "Old choice",
      "is_correct": false,
      "delete": true
    }
  ]
}
```

**Choice Update Rules:**
- Include `id` for existing choices to update
- Omit `id` for new choices to create
- Set `delete: true` to remove existing choices

**Response (200 OK):**
```json
{
  "message": "Question updated successfully",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "exam_id": "123e4567-e89b-12d3-a456-426614174000",
    "question_text": "Updated: What is the average time complexity of binary search?",
    "question_type": "multiple_choice",
    "correct_answer": null,
    "score": 8,
    "choices": []
  }
}
```

---

### 6. Delete Question
Delete an existing question. **Teacher role required.**

**Endpoint:** `DELETE /api/v1/questions/{question_id}`

**Headers:**
```http
Authorization: Bearer <access-token>
```

**Path Parameters:**
- `question_id`: UUID of the question to delete

**Response (200 OK):**
```json
{
  "message": "Question deleted successfully"
}
```

---

### 7. Bulk Create Questions
Create multiple questions for an exam at once. **Teacher role required.**

**Endpoint:** `POST /api/v1/questions/bulk`

**Headers:**
```http
Authorization: Bearer <access-token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "exam_id": "123e4567-e89b-12d3-a456-426614174000",
  "questions": [
    {
      "question_text": "What is a linked list?",
      "question_type": "essay",
      "correct_answer": "A linear data structure where elements are stored in nodes.",
      "score": 10,
      "choices": null
    },
    {
      "question_text": "Which sorting algorithm has O(n log n) complexity?",
      "question_type": "multiple_choice",
      "correct_answer": null,
      "score": 5,
      "choices": [
        {
          "choice_text": "Bubble Sort",
          "is_correct": false
        },
        {
          "choice_text": "Merge Sort",
          "is_correct": true
        }
      ]
    }
  ]
}
```

**Response (201 Created):**
```json
{
  "message": "Questions created successfully",
  "data": [
    {
      "id": "question-1-uuid",
      "exam_id": "123e4567-e89b-12d3-a456-426614174000",
      "question_text": "What is a linked list?",
      "question_type": "essay",
      "correct_answer": "A linear data structure where elements are stored in nodes.",
      "score": 10,
      "choices": []
    }
  ]
}
```

---

### 8. Get Exam Total Score
Get the total score for all questions in an exam.

**Endpoint:** `GET /api/v1/questions/exam/{exam_id}/total-score`

**Headers:**
```http
Authorization: Bearer <access-token>
```

**Path Parameters:**
- `exam_id`: UUID of the exam

**Response (200 OK):**
```json
{
  "message": "Total score retrieved successfully",
  "data": {
    "exam_id": "123e4567-e89b-12d3-a456-426614174000",
    "total_score": 45
  }
}
```

---

## Question Types

### Multiple Choice
- Use `choices` array to define options
- Set `is_correct: true` for the correct choice(s)
- `correct_answer` field is typically null

### Essay
- Use `correct_answer` field for model answer
- `choices` array is null or empty
- Requires manual grading

### True/False
- Use `correct_answer` field: "true" or "false"
- `choices` array is null or empty

## Authentication & Authorization

### Required Headers
```http
Authorization: Bearer <access-token>
```

### Role Requirements
- **Get Questions**: Any authenticated user
- **Get Questions for Student**: Student role (hides correct answers)
- **Create/Update/Delete Questions**: Teacher role
- **Bulk Operations**: Teacher role

## Data Types
- All IDs are UUIDs
- All timestamps use ISO 8601 format (UTC)
- Question types: "multiple_choice", "essay", "true_false"
- Scores are positive integers
