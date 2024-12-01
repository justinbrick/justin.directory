use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, jwk::JwkSet, DecodingKey, Validation};
use reqwest::IntoUrl;
use serde::{Deserialize, Serialize};
use tonic::{Request, Status};

pub struct AuthHandler {
    jwk_set_url: String,
    jwk_set: Mutex<Option<JwkSet>>,
    last_refreshed: Mutex<DateTime<Utc>>,
    pub validation: Arc<Validation>,
}

async fn fetch_jwk_set<T: IntoUrl>(url: T) -> Result<JwkSet, Box<dyn std::error::Error>> {
    let res = reqwest::get(url).await?;
    let text = res.text().await?;

    Ok(serde_json::from_str(text.as_str())?)
}

impl AuthHandler {
    pub fn new(url: impl Into<String>) -> Self {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UserContext {
    email: String,
}

pub async fn authenticate(
    mut req: Request<()>,
    jwk_set: JwkSet,
    validation: Arc<Validation>,
) -> Result<Request<()>, Status> {
    if let Some(auth) = req.metadata().get("authorization") {
        let token_str = auth
            .to_str()
            .map_err(|_| Status::unauthenticated("Invalid token"))?;

        let token = jwk_set
            .keys
            .iter()
            .find_map(|key| {
                decode::<UserContext>(token_str, &DecodingKey::from_jwk(key).ok()?, &validation)
                    .ok()
            })
            .ok_or_else(|| Status::unauthenticated("Invalid token"))?;

        req.extensions_mut().insert(token.claims);
    }

    Ok(req)
}
