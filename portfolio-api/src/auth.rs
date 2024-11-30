use std::{
    env,
    future::Future,
    ops::{Deref, DerefMut},
    pin::Pin,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;
use openid::Client;
use tonic::{Request, Response, Status};
use tower::{Layer, Service};

pub struct MicrosoftAuth {
    client: Option<Client>,
}

impl MicrosoftAuth {
    pub fn new() -> Self {
        Self { client: None }
    }

    pub async fn client(&mut self) -> &Client {
        if let None = self.client {
            let client = Client::discover(
                env::var("MSFT_CLIENT_ID").unwrap(),
                None,
                None,
                "https://login.microsoftonline.com/".try_into().unwrap(),
            )
            .await
            .unwrap();

            self.client = Some(client);
        }

        self.client.as_ref().unwrap()
    }
}

struct UserContext {
    email: String,
}

pub async fn authenticate(
    mut req: Request<()>,
    client: Arc<Client>,
) -> Result<Request<()>, Status> {
    if let Some(auth) = req.metadata().get("authorization") {
        let extensions = req.extensions_mut();
    }

    Ok(req)
}
