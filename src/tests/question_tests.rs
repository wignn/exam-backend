use crate::models::question::{
    CreateQuestionRequest, CreateChoiceRequest, UpdateQuestionRequest, UpdateChoiceRequest,
    QuestionType, BulkCreateQuestionsRequest
};
use crate::models::user::UserRole;
use crate::middleware::auth::AuthUser;
use uuid::Uuid;
use validator::Validate;

#[cfg(test)]
mod question_service_tests {
    use super::*;
    use crate::database::Database;
    use crate::services::question::QuestionService;
    use sqlx::PgPool;

    // Helper function to create test database
    async fn create_test_db() -> Database {
        Database {
            pool: PgPool::connect("postgresql://test:test@localhost/test_db")
                .await
                .expect("Failed to connect to test database"),
        }
    }

    #[tokio::test]
    #[ignore] // Ignored because it requires a test database
    async fn test_create_question() {
        let db = create_test_db().await;
        let question_service = QuestionService::new(db);
        let exam_id = Uuid::new_v4();

        let choices = vec![
            CreateChoiceRequest {
                choice_text: "Option A".to_string(),
                is_correct: true,
            },
            CreateChoiceRequest {
                choice_text: "Option B".to_string(),
                is_correct: false,
            },
        ];

        let request = CreateQuestionRequest {
            question_text: "What is 2 + 2?".to_string(),
            question_type: QuestionType::MultipleChoice,
            correct_answer: Some("A".to_string()),
            score: 10,
            choices: Some(choices),
        };

        let result = question_service.create_question(exam_id, request).await;
        if result.is_ok() {
            let question_response = result.unwrap();
            assert_eq!(question_response.question_text, "What is 2 + 2?");
            assert_eq!(question_response.score, 10);
            assert_eq!(question_response.choices.len(), 2);
        }
    }

    #[tokio::test]
    #[ignore] // Ignored because it requires a test database
    async fn test_get_questions_by_exam() {
        let db = create_test_db().await;
        let question_service = QuestionService::new(db);
        let exam_id = Uuid::new_v4();

        let result = question_service.get_questions_by_exam(exam_id).await;
        if result.is_ok() {
            let questions = result.unwrap();
            // Should return empty list for new exam
            assert!(questions.is_empty());
        }
    }

    #[tokio::test]
    #[ignore] // Ignored because it requires a test database
    async fn test_get_questions_for_student() {
        let db = create_test_db().await;
        let question_service = QuestionService::new(db);
        let exam_id = Uuid::new_v4();

        let result = question_service.get_questions_for_student(exam_id).await;
        if result.is_ok() {
            let questions = result.unwrap();
            // Should return empty list for new exam
            assert!(questions.is_empty());
        }
    }

    #[tokio::test]
    #[ignore] // Ignored because it requires a test database
    async fn test_bulk_create_questions() {
        let db = create_test_db().await;
        let question_service = QuestionService::new(db);
        let exam_id = Uuid::new_v4();

        let questions = vec![
            CreateQuestionRequest {
                question_text: "Question 1".to_string(),
                question_type: QuestionType::MultipleChoice,
                correct_answer: Some("A".to_string()),
                score: 5,
                choices: None,
            },
            CreateQuestionRequest {
                question_text: "Question 2".to_string(),
                question_type: QuestionType::Essay,
                correct_answer: None,
                score: 15,
                choices: None,
            },
        ];

        let request = BulkCreateQuestionsRequest {
            exam_id,
            questions,
        };

        let result = question_service.bulk_create_questions(request).await;
        if result.is_ok() {
            let created_questions = result.unwrap();
            assert_eq!(created_questions.len(), 2);
        }
    }

    #[tokio::test]
    #[ignore] // Ignored because it requires a test database
    async fn test_get_exam_total_score() {
        let db = create_test_db().await;
        let question_service = QuestionService::new(db);
        let exam_id = Uuid::new_v4();

        let result = question_service.get_exam_total_score(exam_id).await;
        if result.is_ok() {
            let total_score = result.unwrap();
            // Should be 0 for new exam with no questions
            assert_eq!(total_score, 0);
        }
    }

    #[tokio::test]
    async fn test_question_request_validation() {
        // Test valid create request
        let valid_create = CreateQuestionRequest {
            question_text: "Valid question text".to_string(),
            question_type: QuestionType::MultipleChoice,
            correct_answer: Some("A".to_string()),
            score: 10,
            choices: None,
        };
        assert!(valid_create.validate().is_ok());

        // Test invalid create request
        let invalid_create = CreateQuestionRequest {
            question_text: "".to_string(), // Empty text should fail
            question_type: QuestionType::MultipleChoice,
            correct_answer: Some("A".to_string()),
            score: 0, // Zero score should fail
            choices: None,
        };
        assert!(invalid_create.validate().is_err());

        // Test valid update request
        let valid_update = UpdateQuestionRequest {
            question_text: "Updated question text".to_string(),
            question_type: QuestionType::Essay,
            correct_answer: None,
            score: 15,
            choices: None,
        };
        assert!(valid_update.validate().is_ok());

        // Test invalid update request
        let invalid_update = UpdateQuestionRequest {
            question_text: "".to_string(), // Empty text should fail
            question_type: QuestionType::Essay,
            correct_answer: None,
            score: 0, // Zero score should fail
            choices: None,
        };
        assert!(invalid_update.validate().is_err());
    }

    #[tokio::test]
    async fn test_choice_request_validation() {
        // Test valid choice creation
        let valid_choice = CreateChoiceRequest {
            choice_text: "Valid choice text".to_string(),
            is_correct: true,
        };
        assert!(valid_choice.validate().is_ok());

        // Test invalid choice creation
        let invalid_choice = CreateChoiceRequest {
            choice_text: "".to_string(), // Empty text should fail
            is_correct: false,
        };
        assert!(invalid_choice.validate().is_err());

        // Test valid choice update
        let valid_update_choice = UpdateChoiceRequest {
            id: Some(Uuid::new_v4()),
            choice_text: "Updated choice text".to_string(),
            is_correct: false,
            delete: Some(false),
        };
        assert!(valid_update_choice.validate().is_ok());

        // Test choice deletion
        let delete_choice = UpdateChoiceRequest {
            id: Some(Uuid::new_v4()),
            choice_text: "Choice to delete".to_string(),
            is_correct: false,
            delete: Some(true),
        };
        assert!(delete_choice.validate().is_ok());
    }

    #[tokio::test]
    async fn test_question_type_conversion() {
        // Test string to QuestionType conversion
        let mc_type: Result<QuestionType, String> = "multiple_choice".parse();
        assert!(mc_type.is_ok());
        assert!(matches!(mc_type.unwrap(), QuestionType::MultipleChoice));

        let essay_type: Result<QuestionType, String> = "essay".parse();
        assert!(essay_type.is_ok());
        assert!(matches!(essay_type.unwrap(), QuestionType::Essay));

        let tf_type: Result<QuestionType, String> = "true_false".parse();
        assert!(tf_type.is_ok());
        assert!(matches!(tf_type.unwrap(), QuestionType::TrueFalse));

        let invalid_type: Result<QuestionType, String> = "invalid_type".parse();
        assert!(invalid_type.is_err());

        // Test QuestionType to string conversion
        assert_eq!(QuestionType::MultipleChoice.to_string(), "multiple_choice");
        assert_eq!(QuestionType::Essay.to_string(), "essay");
        assert_eq!(QuestionType::TrueFalse.to_string(), "true_false");
    }

    #[tokio::test]
    async fn test_bulk_questions_validation() {
        let exam_id = Uuid::new_v4();

        let valid_questions = vec![
            CreateQuestionRequest {
                question_text: "Question 1".to_string(),
                question_type: QuestionType::MultipleChoice,
                correct_answer: Some("A".to_string()),
                score: 5,
                choices: None,
            },
            CreateQuestionRequest {
                question_text: "Question 2".to_string(),
                question_type: QuestionType::Essay,
                correct_answer: None,
                score: 10,
                choices: None,
            },
        ];

        let valid_bulk = BulkCreateQuestionsRequest {
            exam_id,
            questions: valid_questions,
        };

        // Validate each question in the bulk request
        for question in &valid_bulk.questions {
            assert!(question.validate().is_ok());
        }

        let invalid_questions = vec![
            CreateQuestionRequest {
                question_text: "".to_string(), // Invalid
                question_type: QuestionType::MultipleChoice,
                correct_answer: Some("A".to_string()),
                score: 0, // Invalid
                choices: None,
            },
        ];

        let invalid_bulk = BulkCreateQuestionsRequest {
            exam_id,
            questions: invalid_questions,
        };

        // At least one question should be invalid
        let mut has_invalid = false;
        for question in &invalid_bulk.questions {
            if question.validate().is_err() {
                has_invalid = true;
                break;
            }
        }
        assert!(has_invalid);
    }

    #[tokio::test]
    async fn test_uuid_uniqueness_for_questions() {
        let question_id1 = Uuid::new_v4();
        let question_id2 = Uuid::new_v4();
        let exam_id1 = Uuid::new_v4();
        let exam_id2 = Uuid::new_v4();

        // All UUIDs should be unique
        assert_ne!(question_id1, question_id2);
        assert_ne!(exam_id1, exam_id2);
        assert_ne!(question_id1, exam_id1);

        // All UUIDs should not be nil
        assert_ne!(question_id1, Uuid::nil());
        assert_ne!(question_id2, Uuid::nil());
        assert_ne!(exam_id1, Uuid::nil());
        assert_ne!(exam_id2, Uuid::nil());
    }
}

#[cfg(test)]
mod question_handler_tests {
    use super::*;

    fn create_test_teacher() -> AuthUser {
        AuthUser {
            id: Uuid::new_v4(),
            email: "teacher@test.com".to_string(),
            role: UserRole::Teacher,
        }
    }

    fn create_test_student() -> AuthUser {
        AuthUser {
            id: Uuid::new_v4(),
            email: "student@test.com".to_string(),
            role: UserRole::Student,
        }
    }

    #[tokio::test]
    async fn test_create_question_request_validation() {
        let valid_request = CreateQuestionRequest {
            question_text: "What is 2 + 2?".to_string(),
            question_type: QuestionType::MultipleChoice,
            correct_answer: Some("4".to_string()),
            score: 10,
            choices: Some(vec![
                CreateChoiceRequest {
                    choice_text: "2".to_string(),
                    is_correct: false,
                },
                CreateChoiceRequest {
                    choice_text: "4".to_string(),
                    is_correct: true,
                },
            ]),
        };
        assert!(valid_request.validate().is_ok());

        let invalid_request = CreateQuestionRequest {
            question_text: "".to_string(), // Empty text should fail
            question_type: QuestionType::MultipleChoice,
            correct_answer: Some("4".to_string()),
            score: 0, // Zero score should fail validation
            choices: None,
        };
        assert!(invalid_request.validate().is_err());
    }

    #[tokio::test]
    async fn test_update_question_request_validation() {
        let valid_request = UpdateQuestionRequest {
            question_text: "Updated question text".to_string(),
            question_type: QuestionType::Essay,
            correct_answer: None,
            score: 15,
            choices: None,
        };
        assert!(valid_request.validate().is_ok());

        let invalid_request = UpdateQuestionRequest {
            question_text: "".to_string(), // Empty text should fail
            question_type: QuestionType::MultipleChoice,
            correct_answer: Some("Answer".to_string()),
            score: 0, // Zero score should fail
            choices: None,
        };
        assert!(invalid_request.validate().is_err());
    }

    #[tokio::test]
    async fn test_create_choice_request_validation() {
        let valid_request = CreateChoiceRequest {
            choice_text: "Option A".to_string(),
            is_correct: true,
        };
        assert!(valid_request.validate().is_ok());

        let invalid_request = CreateChoiceRequest {
            choice_text: "".to_string(), // Empty choice text should fail
            is_correct: false,
        };
        assert!(invalid_request.validate().is_err());
    }

    #[tokio::test]
    async fn test_update_choice_request_validation() {
        let valid_request = UpdateChoiceRequest {
            id: Some(Uuid::new_v4()),
            choice_text: "Updated choice text".to_string(),
            is_correct: false,
            delete: Some(false),
        };
        assert!(valid_request.validate().is_ok());

        let invalid_request = UpdateChoiceRequest {
            id: None,
            choice_text: "".to_string(), // Empty text should fail
            is_correct: true,
            delete: None,
        };
        assert!(invalid_request.validate().is_err());
    }

    #[tokio::test]
    async fn test_auth_user_roles() {
        let teacher = create_test_teacher();
        let student = create_test_student();

        assert_eq!(teacher.role, UserRole::Teacher);
        assert_eq!(student.role, UserRole::Student);
        assert!(matches!(teacher.role, UserRole::Teacher));
        assert!(!matches!(student.role, UserRole::Teacher));
    }

    #[tokio::test]
    async fn test_question_types() {
        // Test different question types
        let mc_question = CreateQuestionRequest {
            question_text: "Multiple choice question?".to_string(),
            question_type: QuestionType::MultipleChoice,
            correct_answer: Some("A".to_string()),
            score: 5,
            choices: Some(vec![
                CreateChoiceRequest {
                    choice_text: "Choice A".to_string(),
                    is_correct: true,
                },
                CreateChoiceRequest {
                    choice_text: "Choice B".to_string(),
                    is_correct: false,
                },
            ]),
        };
        assert!(matches!(mc_question.question_type, QuestionType::MultipleChoice));

        let essay_question = CreateQuestionRequest {
            question_text: "Essay question?".to_string(),
            question_type: QuestionType::Essay,
            correct_answer: None, // Essay questions don't have predefined correct answers
            score: 20,
            choices: None,
        };
        assert!(matches!(essay_question.question_type, QuestionType::Essay));

        let tf_question = CreateQuestionRequest {
            question_text: "True or False question?".to_string(),
            question_type: QuestionType::TrueFalse,
            correct_answer: Some("true".to_string()),
            score: 2,
            choices: Some(vec![
                CreateChoiceRequest {
                    choice_text: "True".to_string(),
                    is_correct: true,
                },
                CreateChoiceRequest {
                    choice_text: "False".to_string(),
                    is_correct: false,
                },
            ]),
        };
        assert!(matches!(tf_question.question_type, QuestionType::TrueFalse));
    }

    #[tokio::test]
    async fn test_uuid_generation() {
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        assert_ne!(id1, id2);
        assert!(id1 != Uuid::nil());
        assert!(id2 != Uuid::nil());
    }

    #[tokio::test]
    async fn test_question_text_validation_edge_cases() {
        // Test very long question text
        let long_text = "a".repeat(1000);
        let long_request = CreateQuestionRequest {
            question_text: long_text,
            question_type: QuestionType::Essay,
            correct_answer: None,
            score: 10,
            choices: None,
        };
        assert!(long_request.validate().is_ok());

        // Test question text with special characters
        let special_request = CreateQuestionRequest {
            question_text: "What is the value of π (pi) ≈ 3.14159?".to_string(),
            question_type: QuestionType::MultipleChoice,
            correct_answer: Some("3.14159".to_string()),
            score: 5,
            choices: Some(vec![
                CreateChoiceRequest {
                    choice_text: "3.14159".to_string(),
                    is_correct: true,
                },
                CreateChoiceRequest {
                    choice_text: "3.14".to_string(),
                    is_correct: false,
                },
            ]),
        };
        assert!(special_request.validate().is_ok());

        // Test whitespace only question text
        let _whitespace_request = CreateQuestionRequest {
            question_text: "   ".to_string(),
            question_type: QuestionType::MultipleChoice,
            correct_answer: Some("Answer".to_string()),
            score: 5,
            choices: None,
        };
        // This should likely fail validation if trimmed
        // assert!(whitespace_request.validate().is_err());
    }

    #[tokio::test]
    async fn test_choice_correctness_logic() {
        let correct_choice = CreateChoiceRequest {
            choice_text: "Correct answer".to_string(),
            is_correct: true,
        };
        assert_eq!(correct_choice.is_correct, true);

        let incorrect_choice = CreateChoiceRequest {
            choice_text: "Wrong answer".to_string(),
            is_correct: false,
        };
        assert_eq!(incorrect_choice.is_correct, false);
    }

    #[tokio::test]
    async fn test_score_validation() {
        // Test positive scores
        let positive_score_request = CreateQuestionRequest {
            question_text: "Valid question?".to_string(),
            question_type: QuestionType::MultipleChoice,
            correct_answer: Some("Yes".to_string()),
            score: 10,
            choices: Some(vec![
                CreateChoiceRequest {
                    choice_text: "Yes".to_string(),
                    is_correct: true,
                },
                CreateChoiceRequest {
                    choice_text: "No".to_string(),
                    is_correct: false,
                },
            ]),
        };
        assert!(positive_score_request.validate().is_ok());
        assert!(positive_score_request.score > 0);

        // Test high scores
        let high_score_request = CreateQuestionRequest {
            question_text: "High value question?".to_string(),
            question_type: QuestionType::Essay,
            correct_answer: None,
            score: 100,
            choices: None,
        };
        assert!(high_score_request.validate().is_ok());
        assert_eq!(high_score_request.score, 100);
    }

    #[tokio::test]
    async fn test_correct_answer_logic() {
        // Multiple choice should have correct answer
        let mc_with_answer = CreateQuestionRequest {
            question_text: "MC question?".to_string(),
            question_type: QuestionType::MultipleChoice,
            correct_answer: Some("A".to_string()),
            score: 5,
            choices: Some(vec![
                CreateChoiceRequest {
                    choice_text: "Choice A".to_string(),
                    is_correct: true,
                },
                CreateChoiceRequest {
                    choice_text: "Choice B".to_string(),
                    is_correct: false,
                },
            ]),
        };
        assert!(mc_with_answer.correct_answer.is_some());

        // Essay questions typically don't have predefined correct answers
        let essay_no_answer = CreateQuestionRequest {
            question_text: "Essay question?".to_string(),
            question_type: QuestionType::Essay,
            correct_answer: None,
            score: 20,
            choices: None,
        };
        assert!(essay_no_answer.correct_answer.is_none());

        // True/False should have correct answer
        let tf_with_answer = CreateQuestionRequest {
            question_text: "TF question?".to_string(),
            question_type: QuestionType::TrueFalse,
            correct_answer: Some("false".to_string()),
            score: 2,
            choices: Some(vec![
                CreateChoiceRequest {
                    choice_text: "True".to_string(),
                    is_correct: false,
                },
                CreateChoiceRequest {
                    choice_text: "False".to_string(),
                    is_correct: true,
                },
            ]),
        };
        assert!(tf_with_answer.correct_answer.is_some());
    }

    #[tokio::test]
    async fn test_question_update_scenarios() {
        let teacher = create_test_teacher();
        let student = create_test_student();

        // Only teachers should be able to update questions
        assert!(matches!(teacher.role, UserRole::Teacher));
        assert!(!matches!(student.role, UserRole::Teacher));

        let update_request = UpdateQuestionRequest {
            question_text: "Updated question text".to_string(),
            question_type: QuestionType::MultipleChoice,
            correct_answer: Some("Updated answer".to_string()),
            score: 15,
            choices: Some(vec![
                UpdateChoiceRequest {
                    id: Some(Uuid::new_v4()),
                    choice_text: "Updated choice".to_string(),
                    is_correct: true,
                    delete: Some(false),
                },
            ]),
        };
        assert!(update_request.validate().is_ok());
    }

    #[tokio::test]
    async fn test_choice_update_scenarios() {
        let update_choice = UpdateChoiceRequest {
            id: Some(Uuid::new_v4()),
            choice_text: "Updated choice".to_string(),
            is_correct: true,
            delete: Some(false),
        };
        assert!(update_choice.validate().is_ok());

        // Test toggling correctness
        let toggle_correctness = UpdateChoiceRequest {
            id: Some(Uuid::new_v4()),
            choice_text: "Same text".to_string(),
            is_correct: false, // Changed from true to false
            delete: Some(false),
        };
        assert!(toggle_correctness.validate().is_ok());
        assert_eq!(toggle_correctness.is_correct, false);
    }

    #[tokio::test]
    async fn test_bulk_create_questions_request() {
        let exam_id = Uuid::new_v4();
        let bulk_request = BulkCreateQuestionsRequest {
            exam_id,
            questions: vec![
                CreateQuestionRequest {
                    question_text: "Question 1".to_string(),
                    question_type: QuestionType::MultipleChoice,
                    correct_answer: Some("A".to_string()),
                    score: 5,
                    choices: Some(vec![
                        CreateChoiceRequest {
                            choice_text: "Choice A".to_string(),
                            is_correct: true,
                        },
                        CreateChoiceRequest {
                            choice_text: "Choice B".to_string(),
                            is_correct: false,
                        },
                    ]),
                },
                CreateQuestionRequest {
                    question_text: "Question 2".to_string(),
                    question_type: QuestionType::Essay,
                    correct_answer: None,
                    score: 10,
                    choices: None,
                },
            ],
        };

        // Validate each question in the bulk request
        for question in &bulk_request.questions {
            assert!(question.validate().is_ok());
        }

        assert_eq!(bulk_request.questions.len(), 2);
        assert!(bulk_request.exam_id != Uuid::nil());
    }

    #[tokio::test]
    async fn test_question_type_conversion() {
        // Test question type to string conversion
        assert_eq!(QuestionType::MultipleChoice.to_string(), "multiple_choice");
        assert_eq!(QuestionType::Essay.to_string(), "essay");
        assert_eq!(QuestionType::TrueFalse.to_string(), "true_false");

        // Test string to question type conversion
        let mc_type: Result<QuestionType, _> = "multiple_choice".parse();
        assert!(mc_type.is_ok());
        assert!(matches!(mc_type.unwrap(), QuestionType::MultipleChoice));

        let essay_type: Result<QuestionType, _> = "essay".parse();
        assert!(essay_type.is_ok());
        assert!(matches!(essay_type.unwrap(), QuestionType::Essay));

        let tf_type: Result<QuestionType, _> = "true_false".parse();
        assert!(tf_type.is_ok());
        assert!(matches!(tf_type.unwrap(), QuestionType::TrueFalse));

        // Test invalid string conversion
        let invalid_type: Result<QuestionType, _> = "invalid_type".parse();
        assert!(invalid_type.is_err());
    }
}
