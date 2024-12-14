use std::{
    future::{Future, IntoFuture},
    pin::Pin,
    sync::{Arc, Mutex},
};

use axum::{
    async_trait,
    body::Body,
    extract::{FromRequestParts, Request},
    response::Response,
};
use chrono::{DateTime, Utc};
use http::{request::Parts, HeaderValue, StatusCode};
use jsonwebtoken::{decode, jwk::JwkSet, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tower_http::auth::AsyncAuthorizeRequest;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserContext {
    sub: String,
    oid: String,
    tid: String,
    iss: String,
    scp: String,
}

#[derive(Clone)]
pub struct User(pub UserContext);

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Some(context) = parts.extensions.get::<UserContext>() else {
            return Err((StatusCode::UNAUTHORIZED, "user not signed in"));
        };

        Ok(User(context.clone()))
    }
}

pub struct AuthHandler {
    jwk_set_url: String,
    jwk_set: Mutex<Option<JwkSet>>,
    last_refreshed: Mutex<DateTime<Utc>>,
    validation: Arc<Validation>,
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

    pub fn state(&self) -> (Option<JwkSet>, DateTime<Utc>) {
        (
            self.jwk_set.lock().unwrap().to_owned(),
            self.last_refreshed.lock().unwrap().to_owned(),
        )
    }

    fn update(&self, jwk_set: JwkSet) {
        *self.jwk_set.lock().unwrap() = Some(jwk_set);
        *self.last_refreshed.lock().unwrap() = Utc::now();
    }

    pub async fn jwks(&self) -> Option<JwkSet> {
        let (jwk_set, last_refreshed) = self.state();
        if jwk_set.is_none() || last_refreshed.signed_duration_since(Utc::now()).num_hours() > 24 {
            let res = reqwest::get(&self.jwk_set_url).await.ok()?;
            let text = res.text().await.ok()?;
            let jwk_set: JwkSet = serde_json::from_str(text.as_str()).ok()?;

            self.update(jwk_set.clone());
            return Some(jwk_set);
        }

        jwk_set
    }
}

#[derive(Clone)]
pub struct MicrosoftAuth {
    auth: Arc<AuthHandler>,
}

impl MicrosoftAuth {
    pub fn new(auth: Arc<AuthHandler>) -> Self {
        Self { auth }
    }
}

impl<B> AsyncAuthorizeRequest<B> for MicrosoftAuth
where
    B: Send + 'static,
{
    type RequestBody = B;
    type ResponseBody = Body;
    type Future = Pin<
        Box<dyn Future<Output = Result<Request<B>, Response<Self::ResponseBody>>> + Send + 'static>,
    >;

    fn authorize(&mut self, req: Request<B>) -> Self::Future {
        let auth = self.auth.clone();
        let fut = auth_logic(req, auth);
        Box::pin(fut.into_future())
    }
}

async fn auth_logic<B>(
    mut req: Request<B>,
    auth: Arc<AuthHandler>,
) -> Result<Request<B>, Response<Body>> {
    let Some(authorization) = req.headers().get(http::header::AUTHORIZATION) else {
        return Ok(req);
    };
    let auth_str = extract_auth_header(&authorization)?;
    let jwks = fetch_jwks(&auth).await?;
    let context = decode_token(auth_str.as_str(), &jwks, &auth.validation)?;
    req.extensions_mut().insert(User(context));

    Ok(req)
}

fn extract_auth_header(auth: &HeaderValue) -> Result<String, Response<Body>> {
    let auth_str = auth
        .to_str()
        .map_err(|_| bad_auth("failed to parse auth header"))?;
    let binding = auth_str.split(' ').collect::<Vec<_>>();
    let [scheme, auth_str] = binding.as_slice() else {
        return Err(bad_auth("invalid auth header"));
    };
    if *scheme != "Bearer" {
        return Err(bad_auth("invalid auth scheme"));
    }

    Ok(auth_str.to_string())
}

async fn fetch_jwks(auth: &AuthHandler) -> Result<JwkSet, Response<Body>> {
    let jwks = auth
        .jwks()
        .await
        .ok_or_else(|| bad_auth("failed to fetch jwks"))?;
    Ok(jwks)
}

fn decode_token(
    auth_str: &str,
    jwks: &JwkSet,
    validation: &Validation,
) -> Result<UserContext, Response<Body>> {
    let context = jwks
        .keys
        .iter()
        .find_map(|jwk| {
            decode::<UserContext>(auth_str, &DecodingKey::from_jwk(jwk).ok()?, validation).ok()
        })
        .ok_or_else(|| bad_auth("invalid token"))?;
    Ok(context.claims)
}

fn bad_auth(reason: &'static str) -> Response<Body> {
    Response::builder()
        .status(401)
        .body(reason.to_owned().into())
        .expect("issue building `http::Response`")
}
