use crate::models::api_response::ApiResponse;
use http::StatusCode;

pub struct ApiResponseBuilder<T> {
    status: StatusCode,
    message: String,
    data: Option<T>,
}

impl<T> ApiResponseBuilder<T> {
    pub fn new() -> Self {
        ApiResponseBuilder {
            status: StatusCode::OK,
            message: String::new(),
            data: None,
        }
    }

    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = message.to_owned();
        self
    }

    pub fn data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    pub fn build(self) -> ApiResponse<T> {
        ApiResponse {
            status: self.status.as_u16(),
            message: self.message,
            data: self.data,
        }
    }
}
