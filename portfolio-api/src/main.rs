use std::sync::Arc;

use auth::{authenticate, MicrosoftAuth};
use openid::Client;
use portfolio::{pb::portfolio_server::PortfolioServer, PortfolioService};
use tonic::transport::Server;
use tonic_async_interceptor::async_interceptor;

mod auth;
mod portfolio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Arc::new(
        Client::discover(
            "".into(),
            None,
            None,
            "https://login.microsoftonline.com/".try_into().unwrap(),
        )
        .await
        .unwrap(),
    );
    let portfolio = PortfolioService {};
    let service = PortfolioServer::new(portfolio);
    let layer = tower::ServiceBuilder::new()
        .layer(async_interceptor(move |req: tonic::Request<()>| {
            authenticate(req, client.clone())
        }))
        .into_inner();

    Server::builder()
        .layer(layer)
        .add_service(service)
        .serve("[::1]:50051".parse().unwrap())
        .await?;

    Ok(())
}
