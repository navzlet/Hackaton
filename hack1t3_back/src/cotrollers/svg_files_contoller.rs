use std::sync::Mutex;

use actix_web::error::InternalError;
use actix_web::web;


use actix_web::web::{Data, Json};
use crate::db::Db;
use crate::models::CsvFile;
use crate::StdErr;
use crate::utils::to_internal_error;

#[actix_web::get("/get_csv_files_list")]
pub async fn get_csv_files_list(db: web::Data<Db>) -> Result<Json<Vec<CsvFile>>, InternalError<StdErr>> {
    db.list_csv_files()
        .await
        .map(Json)
        .map_err(to_internal_error)
}

#[actix_web::get("/exist_csv_file")]
pub async fn exist_csv_file(db: web::Data<Db>) -> Result<Json<bool>, InternalError<StdErr>> {
    db.exist_file_csv("docs_АОСР.csv")
        .await
        .map(Json)
        .map_err(to_internal_error)
}