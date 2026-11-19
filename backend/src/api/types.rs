use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.into()),
        }
    }
}

pub struct ApiError {
    pub status: StatusCode,
    pub message: String,
}

impl ApiError {
    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: msg.into(),
        }
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: msg.into(),
        }
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: msg.into(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(ApiResponse::<()>::err(self.message));
        (self.status, body).into_response()
    }
}

impl From<rusqlite::Error> for ApiError {
    fn from(err: rusqlite::Error) -> Self {
        tracing::error!("Database error: {}", err);
        ApiError::internal("An internal database error occurred")
    }
}

impl From<r2d2::Error> for ApiError {
    fn from(err: r2d2::Error) -> Self {
        tracing::error!("Connection pool error: {}", err);
        ApiError::internal("Failed to acquire database connection")
    }
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<usize>,
    pub page_size: Option<usize>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_desc: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: usize,
    pub page_size: usize,
    pub total_pages: usize,
}
