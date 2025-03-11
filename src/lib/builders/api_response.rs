use crate::models::api_response::ApiResponse;
use rocket::http::Status;

pub struct ApiResponseBuilder<T> {
    status: Status,
    message: String,
    data: Option<T>,
}

impl<T> ApiResponseBuilder<T> {
    pub fn new() -> Self {
        ApiResponseBuilder {
            status: Status::Ok,
            message: String::new(),
            data: None,
        }
    }

    pub fn status(mut self, status: Status) -> Self {
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
            status: self.status.code,
            message: self.message,
            data: self.data,
        }
    }
}
