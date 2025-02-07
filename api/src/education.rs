use axum::{extract::State, routing::get, Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{auth::User, AppState, PORTFOLIO_TABLE};

#[derive(Debug, Serialize, Deserialize)]
struct Education {
    #[serde(skip_serializing_if = "Option::is_none")]
    school: Option<String>,
    degree: String,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
}

#[axum_macros::debug_handler]
async fn get_educations(
    maybe_user: Option<User>,
    State(AppState { dynamo }): State<AppState>,
) -> Json<Vec<Education>> {
    let educations = dynamo
        .query()
        .table_name(PORTFOLIO_TABLE)
        .key_condition_expression("#r = :r")
        .expression_attribute_names("#r", "resource")
        .send()
        .await;

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
            degree: "Bachelor of Science in Computer Science".into(),
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

impl<T> EducationRoutable for Router<T>
where
    T: Send + Clone + Sync + 'static,
{
    fn route_education(self) -> Self {
        self.route("/education", get(get_educations))
    }
}
