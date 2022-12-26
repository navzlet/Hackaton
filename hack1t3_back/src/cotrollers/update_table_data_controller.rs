use std::sync::Mutex;
use actix_web::error::InternalError;
use actix_web::web;

use actix_web::web::{Data, Json};
use serde::Serialize;
use crate::{Db, StdErr};
use crate::models::{CellNewValue, CellNewValueRequest, VerifiedRow};
use crate::utils::to_internal_error;

#[actix_web::post("/update_verified_row")]
pub async fn update_verified_row(verified_row: Json<VerifiedRow>, db: web::Data<Db>) -> Result<Json<VerifiedRow>, InternalError<StdErr>> {
    db.update_verified_row(&verified_row)
        .await
        .map_err(to_internal_error)?;
    Ok(verified_row)
}

#[actix_web::post("/update_cell_new_value")]
pub async fn update_cell_new_value(cell_new_value_request: Json<CellNewValueRequest>, db: web::Data<Db>) -> Result<Json<CellNewValueRequest>, InternalError<StdErr>> {
    let document_header = db.get_document_header(
        cell_new_value_request.column_name.as_str(),
        cell_new_value_request.csv_file_uuid.as_str())
        .await
        .map_err(to_internal_error)?;


    db.update_cell_new_value(&CellNewValue {
        csv_file_uuid: cell_new_value_request.csv_file_uuid.clone(),
        column_idx: document_header.column_idx,
        pdf_row_uuid: cell_new_value_request.pdf_row_uuid.clone(),
        new_value: cell_new_value_request.new_value.clone()
    }).await
        .map_err(to_internal_error)?;

    Ok(cell_new_value_request)
}