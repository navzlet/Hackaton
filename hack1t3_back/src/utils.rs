


use actix_web::error::InternalError;
use actix_web::http::StatusCode;



use crate::{StdErr};

pub fn to_internal_error(e: StdErr) -> InternalError<StdErr> {
    InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR)
}