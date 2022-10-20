
use rocket::http;
use rocket::request;
use rocket::response;
use std::io::Cursor;

pub enum ExecutionResult {
    Success(String),
    ModuleResolutionError,
    RuntimeExecutionError
}

impl<'a> response::Responder<'a, 'a> for ExecutionResult {
    fn respond_to(self, _: &request::Request) -> response::Result<'a> {
        match self {
            Self::Success(json) => {
                response::Response::build()
                    .header(http::ContentType::JSON)
                    .status(http::Status::Ok)
                    .sized_body(json.len(), Cursor::new(json))
                    .ok()
            },
            Self::ModuleResolutionError => {
                let err = "Module resolution error";

                response::Response::build()
                    .header(http::ContentType::JSON)
                    .status(http::Status::InternalServerError)
                    .sized_body(err.len(), Cursor::new(err))
                    .ok()
            },
            Self::RuntimeExecutionError => {
                let err = "Runtime execution error";

                response::Response::build()
                    .header(http::ContentType::JSON)
                    .status(http::Status::InternalServerError)
                    .sized_body(err.len(), Cursor::new(err))
                    .ok()
            }
        }
    }
}
