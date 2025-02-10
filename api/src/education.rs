use aws_sdk_dynamodb::types::AttributeValue;
use axum::{extract::State, routing::get, Json, Router};
use chrono::{DateTime, Utc};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{auth::User, AppState, PORTFOLIO_TABLE};

#[derive(Debug, Serialize, Deserialize)]
struct Education {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Uuid>,
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
) -> Result<Json<Vec<Education>>, (StatusCode, &'static str)> {
    let educations = dynamo
        .query()
        .table_name(PORTFOLIO_TABLE)
        .key_condition_expression("#res = :res")
        .expression_attribute_names("#res", "resource")
        .expression_attribute_values(":res", AttributeValue::S("education".to_string()))
        .send()
        .await
        .map_err(|err| {
            tracing::error!("failed to query dynamo: {err}");
            if let Some(body) = err.raw_response().map(|res| res.body()) {
                tracing::error!("error response: {:?}", body);
            }
            (StatusCode::INTERNAL_SERVER_ERROR, "failed to query dynamo")
        })?;

    let Some(items) = educations.items else {
        tracing::warn!("no educations found in dynamo");
        return Ok(Json(vec![]));
    };

    let educations: Vec<Education> = serde_dynamo::from_items(items).map_err(|err| {
        tracing::error!("failed to parse dynamo response: {err}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to parse dynamo response",
        )
    })?;

    if let Some(User(_user)) = maybe_user {
        return Ok(Json(educations));
    }

    Ok(Json(
        educations
            .into_iter()
            .map(|mut education| {
                education.school = None;
                education
            })
            .collect(),
    ))
}

#[axum_macros::debug_handler]
/// Adds education to the database
async fn add_education(
    User(user): User,
    State(AppState { dynamo }): State<AppState>,
    Json(education): Json<Education>,
) -> Result<Json<Education>, (StatusCode, &'static str)> {
    return Ok(Json(education));
}

pub trait EducationRoutable {
    fn route_education(self) -> Self;
}

impl EducationRoutable for Router<AppState> {
    fn route_education(self) -> Self {
        self.route("/education", get(get_educations).post(add_education))
    }
}
