use axum::http::StatusCode;

pub struct HealthCheckController;

impl HealthCheckController {
    pub async fn health_check() -> StatusCode {
        StatusCode::OK
    }
}
