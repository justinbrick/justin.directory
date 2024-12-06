use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use jsonwebtoken::{
    decode,
    jwk::{Jwk, JwkSet},
    DecodingKey, Validation,
};
use serde::{Deserialize, Serialize};

pub struct AuthHandler {
    jwk_set_url: String,
    jwk_set: Mutex<Option<JwkSet>>,
    last_refreshed: Mutex<DateTime<Utc>>,
    pub validation: Arc<Validation>,
}

#[derive(Clone)]
pub struct AuthenticationService {}

impl AuthHandler {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            jwk_set_url: url.into(),
            jwk_set: Mutex::new(None),
            last_refreshed: Mutex::new(DateTime::<Utc>::MIN_UTC),
            validation: Arc::new(Validation::new(jsonwebtoken::Algorithm::RS256)),
        }
    }

    fn set_jwk_set(&self, jwk_set: JwkSet) {
        *self.jwk_set.lock().unwrap() = Some(jwk_set);
    }

    fn tick_last_refresh(&self) {
        *self.last_refreshed.lock().unwrap() = Utc::now();
    }

    fn last_refresh(&self) -> DateTime<Utc> {
        *self.last_refreshed.lock().unwrap()
    }

    fn jwk_set(&self) -> Option<JwkSet> {
        self.jwk_set.lock().unwrap().clone()
    }

    pub async fn get_jwk_set(&self) -> Option<JwkSet> {
        // as i understand, after reading docs, this is not ideal.
        // i can do this lock-free using thread local, but don't want to take the time to do it.
        // please note if you are reading this, i would do this differently had i more time to spend on this.
        let jwk_set = self.jwk_set();
        let last_refreshed = self.last_refresh();

        if jwk_set.is_none() || last_refreshed.signed_duration_since(Utc::now()).num_hours() > 24 {
            let res = reqwest::get(self.jwk_set_url.clone()).await.ok()?;
            let text = res.text().await.ok()?;
            let jwk_set = serde_json::from_str::<JwkSet>(text.as_str()).ok()?;

            self.set_jwk_set(jwk_set.clone());
            self.tick_last_refresh();
            return Some(jwk_set);
        }

        jwk_set
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UserContext {
    sub: String,
    oid: String,
}
