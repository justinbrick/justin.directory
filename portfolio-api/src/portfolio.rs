use pb::{Education, Empty, GetEducationResponse};
use tonic::{Request, Response, Status};

pub mod pb {
    tonic::include_proto!("directory.justin.portfolio");
}

pub struct PortfolioService {}

#[tonic::async_trait]
impl pb::portfolio_server::Portfolio for PortfolioService {
    async fn get_education(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<GetEducationResponse>, Status> {
        Ok(Response::new(GetEducationResponse {
            education: vec![Education {
                school: Some("Yeah...".to_string()),
                degree: "I have a degree".to_string(),
                major: "In something".to_string(),
                start_date: "2010".to_string(),
                end_date: "2014".to_string(),
                gpa: 3.8,
            }],
        }))
    }
}
