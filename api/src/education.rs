use axum::{routing::get, Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::auth::User;

#[derive(Debug, Serialize, Deserialize)]
struct Education {
    #[serde(skip_serializing_if = "Option::is_none")]
    school: Option<String>,
    degree: String,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
}

#[axum_macros::debug_handler]
async fn get_educations(maybe_user: Option<User>) -> Json<Vec<Education>> {
    if let Some(User(_user)) = maybe_user {
        return Json(vec![
            Education {
                school: Some("University of Washington".to_string()),
                degree: "Bachelor of Science in Computer Science".to_string(),
                from: Utc::now(),
                to: Utc::now(),
            },
            Education {
                school: Some("University of Washington".to_string()),
                degree: "Master of Science in Computer Science".to_string(),
                from: Utc::now(),
                to: Utc::now(),
            },
        ]);
    }
    Json(vec![
        Education {
            school: Some("University of Washington".to_string()),
            degree: "Bachelor of Science in Computer Science".to_string(),
            from: Utc::now(),
            to: Utc::now(),
        },
        Education {
            school: Some("University of Washington".to_string()),
            degree: "Master of Science in Computer Science".to_string(),
            from: Utc::now(),
            to: Utc::now(),
        },
    ])
}

async fn get_educations_no_auth() -> Json<Vec<Education>> {
    Json(vec![
        Education {
            school: Some("University of Washington".to_string()),
            degree: "Bachelor of Science in Computer Science".to_string(),
            from: Utc::now(),
            to: Utc::now(),
        },
        Education {
            school: Some("University of Washington".to_string()),
            degree: "Master of Science in Computer Science".to_string(),
            from: Utc::now(),
            to: Utc::now(),
        },
    ])
}

pub trait EducationRoutable {
    fn route_education(self) -> Self;
}

impl EducationRoutable for Router {
    fn route_education(self) -> Self {
        self.route("/education", get(get_educations))
            .route("/education/no_auth", get(get_educations_no_auth))
    }
}
