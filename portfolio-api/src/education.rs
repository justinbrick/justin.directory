use axum::Router;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Education {
    #[serde(skip_serializing_if = "Option::is_none")]
    school: Option<String>,
    degree: String,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
}

pub trait EducationRoutable {
    fn route_education(self) -> Self;
}

impl EducationRoutable for Router {
    fn route_education(self) -> Self {
        self
    }
}
