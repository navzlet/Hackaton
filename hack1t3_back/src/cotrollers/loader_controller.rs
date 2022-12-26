


use std::sync::Mutex;

use actix_web::{HttpResponse, Responder, web};
use actix_web::web::Data;


use crate::{CsvFile, Db, StdErr};
use crate::models::{DocumentCell, DocumentHeader, PdfRow};

use itertools::Itertools;

#[actix_web::get("/load_csv")]
pub async fn load_scv(db : web::Data<Db>) -> Result<impl Responder, StdErr> {
    let base_url = std::env::var("BASE_URL")?;

    let list_csv_var = std::env::var("LIST_CSV")?;
    let list_csv: Vec<&str> = list_csv_var.split(",").collect_vec();

    let list_pdf_dir_var = std::env::var("LIST_PDF_DIR")?;
    let list_pdf_dir: Vec<&str> = list_pdf_dir_var.split(",").collect_vec();

    for (c, csv_filename) in list_csv.into_iter().enumerate() {

        let exists;
        {
            exists = db.exist_file_csv(csv_filename).await?;
        }

        if !exists {
            let csv_file = format!("{}{}", base_url, csv_filename);
            let client = reqwest::ClientBuilder::new().danger_accept_invalid_certs(true)
                .build().expect("invalid make client");
            let res = client.get(csv_file).send().await?;
            let data = res.text_with_charset("windows-1251").await.expect("invalid get");
            //let reader = read_windows1251_file(data);

            let mut csv_reader = csv::ReaderBuilder::new()
                .delimiter(b';')
                .flexible(true)
                .from_reader(data.as_bytes());


            let csv_file = CsvFile::new(csv_filename);

            db.save_file_csv(&csv_file)
                .await?;

            let header = csv_reader.headers()?;

            let count_columns = header.len();

            for (i, header) in header.iter().enumerate() {
                db.save_document_header(&DocumentHeader {
                    csv_file_uuid: csv_file.uuid.to_string(),
                    header_name: header.to_string(),
                    column_idx: i as u16
                }).await?;
            }

            for (i, row) in csv_reader.records().into_iter().enumerate() {
                let row = row?;

                let pdf_file_name = row.get(row.len() - 3usize)
                    .ok_or(StdErr::from(format!("invalid read pdf file name, {}, {}", csv_filename, i)))?;

                let pdf_url = format!("{}{}{}", base_url, list_pdf_dir[c], pdf_file_name);

                let pdf_row = PdfRow::new(pdf_url.as_str());
                db.save_pdf_row(&pdf_row)
                    .await?;


                let mut row = row.iter().map(|v| v).collect_vec();
                row.resize_with(count_columns, || "");

                for (j, value) in row.into_iter().enumerate() {
                    db.save_document_cell(&DocumentCell {
                        csv_file_uuid: csv_file.uuid.clone(),
                        column_idx: j.clone() as u16,
                        pdf_row_uuid: pdf_row.uuid.clone(),
                        value: value.to_string(),
                        new_value: None
                    }).await?;
                }
            }
        }
    }

    Ok(HttpResponse::Ok())
}
