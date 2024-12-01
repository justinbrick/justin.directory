use std::sync::Arc;

use auth::{authenticate, AuthHandler};
use portfolio::{pb::portfolio_server::PortfolioServer, PortfolioService};
use tonic::transport::Server;
use tonic_async_interceptor::async_interceptor;

mod auth;
mod portfolio;

async fn authenticate_wrapper(
    req: tonic::Request<()>,
) -> Result<tonic::Request<()>, tonic::Status> {
    let jwk_set = Arc::new(client.jwk_set().await.unwrap());
    let validation = Arc::new(client.validation().await.unwrap());

    authenticate(req, jwk_set, validation).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handler =
        AuthHandler::new("https://login.microsoftonline.com/organizations/discovery/v2.0/keys");
    let portfolio = PortfolioService {};
    let service = PortfolioServer::new(portfolio);
    let layer = tower::ServiceBuilder::new()
        .layer(async_interceptor(|req: tonic::Request<()>| {
            authenticate(req)
        }))
        .into_inner();

    Server::builder()
        .layer(layer)
        .add_service(service)
        .serve("[::1]:50051".parse().unwrap())
        .await?;

    Ok(())
}
