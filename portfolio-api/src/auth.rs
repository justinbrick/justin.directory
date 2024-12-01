use std::{
    env,
    fmt::Debug,
    future::Future,
    ops::{Deref, DerefMut},
    pin::Pin,
    sync::{Arc, Mutex},
};

use chrono::{DateTime, Utc};
use http::Uri;
use jsonwebtoken::{decode, jwk::JwkSet, Validation};
use reqwest::IntoUrl;
use serde::{Deserialize, Serialize};
use tonic::{Request, Response, Status};
use tower::{Layer, Service};

pub struct AuthHandler {
    jwk_set_url: Uri,
    jwk_set: Mutex<Option<JwkSet>>,
    last_refreshed: Mutex<DateTime<Utc>>,
    validation: Arc<Validation>,
}

async fn fetch_jwk_set<T: IntoUrl>(url: T) -> Result<JwkSet, Box<dyn std::error::Error>> {
    let res = reqwest::get(url).await?;
    let text = res.text().await?;

    Ok(serde_json::from_str(text.as_str())?)
}

impl AuthHandler {
    pub fn new(url: impl Into<Uri>) -> Self {
        Self {
            jwk_set_url: url.into(),
            jwk_set: Mutex::new(None),
            last_refreshed: Mutex::new(DateTime::<Utc>::MIN_UTC),
            validation: Arc::new(Validation::new(jsonwebtoken::Algorithm::RS256)),
        }
    }

    pub async fn jwk_set(&self) -> Option<JwkSet> {
        let mut jwk_set = self.jwk_set.lock().unwrap();
        let mut last_refreshed = self.last_refreshed.lock().unwrap();

        if jwk_set.is_none() || last_refreshed.signed_duration_since(Utc::now()).num_hours() > 24 {
            *jwk_set = Some(fetch_jwk_set(self.jwk_set_url.to_string()).await.unwrap());
            *last_refreshed = Utc::now();
        }

        jwk_set.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct UserContext {
    email: String,
}

pub async fn authenticate(
    mut req: Request<()>,
    jwk_set: Arc<JwkSet>,
    validation: Arc<Validation>,
) -> Result<Request<()>, Status> {
    if let Some(auth) = req.metadata().get("authorization") {
        let token_str = auth
            .to_str()
            .map_err(|_| Status::unauthenticated("Invalid token"))?;

        let token =
            decode::<UserContext>(token_str, &jwk_set.keys.iter().find(predicate), &validation)
                .map_err(|_| Status::unauthenticated("Invalid token"))?;
    }

    Ok(req)
}
