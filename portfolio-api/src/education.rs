use axum::Router;

struct Education {}

pub trait EducationRoutable {
    fn route_education(self) -> Self;
}

impl EducationRoutable for Router {
    fn route_education(self) -> Self {
        self
    }
}
