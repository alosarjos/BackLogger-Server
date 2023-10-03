use axum::{
    routing::{get, IntoMakeService},
    Router as AxumRouter,
};

use super::controller::HealthCheckController;

pub struct Router {
    router: AxumRouter,
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

impl Router {
    pub fn new() -> Self {
        Self {
            router: AxumRouter::new()
                .route("/health_check", get(HealthCheckController::health_check)),
        }
    }

    pub fn get_as_service(&self) -> IntoMakeService<axum::Router> {
        self.router.clone().into_make_service()
    }
}
