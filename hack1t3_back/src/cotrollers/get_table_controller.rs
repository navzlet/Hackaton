use std::borrow::{Borrow, BorrowMut};
use std::sync::Mutex;
use actix_web::error::InternalError;
use actix_web::web;
use actix_web::web::{Data, Json};
use itertools::Itertools;
use crate::{Db, StdErr};
use crate::models::{CsvTable, DocumentCell, DocumentCellResponse, PdfRow};
use crate::utils::to_internal_error;

#[actix_web::get("/get_table/{uuid}")]
pub async fn get_table(uuid: web::Path<String>, db: web::Data<Db>) -> Result<Json<CsvTable>, InternalError<StdErr>> {
    let uuid = uuid.into_inner();


    let document_headers = db
        .get_document_headers(uuid.as_str())
        .await
        .map_err(to_internal_error)?;
    let headers_names = document_headers.into_iter().map(|value| value.header_name).collect_vec();

    let cells = db
        .get_document_cells(uuid.as_str())
        .await
        .map_err(to_internal_error)?;

    let mut rows_info: Vec<PdfRow> = Vec::new();
    let mut data: Vec<Vec<DocumentCellResponse>> = Vec::new();

    for (key, group) in &cells.into_iter()
        .sorted_by(|e1, e2| e1.pdf_row_uuid.cmp(&e2.pdf_row_uuid))
        .group_by(|cell | cell.pdf_row_uuid.clone()) {
        let row = group.sorted_by_key(|key| key.column_idx).collect_vec();
        let row = row.iter().map(|cell| {
            DocumentCellResponse {
                csv_file_uuid: cell.csv_file_uuid.clone(),
                column_name: headers_names.get(cell.column_idx  as usize).expect("invalid column").to_string(),
                pdf_row_uuid: cell.pdf_row_uuid.clone(),
                value: cell.value.clone(),
                new_value: cell.new_value.clone()
            }
        }).collect_vec();
        data.push(row);

        let pdf_row = db
            .get_pdf_row(key.as_str())
            .await
            .map_err(to_internal_error)?;
        rows_info.push(pdf_row);
    }

    Ok(Json(CsvTable {
        headers: headers_names,
        rows_info,
        data
    }))
}