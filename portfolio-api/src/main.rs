use std::sync::Arc;

use auth::{authenticate, AuthHandler};
use portfolio::{pb::portfolio_server::PortfolioServer, PortfolioService};
use tonic::{transport::Server, Status};
use tonic_async_interceptor::async_interceptor;

mod auth;
mod portfolio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handler = Arc::new(AuthHandler::new(
        "https://login.microsoftonline.com/organizations/discovery/v2.0/keys",
    ));
    let portfolio = PortfolioService {};
    let service = PortfolioServer::new(portfolio);
    let layer = tower::ServiceBuilder::new()
        .layer(async_interceptor(move |req: tonic::Request<()>| {
            authenticate(req, handler.clone())
        }))
        .into_inner();

    Server::builder()
        .layer(layer)
        .add_service(service)
        .serve("[::1]:50051".parse().unwrap())
        .await?;

    Ok(())
}
