use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CsvFile {
    pub uuid : String,
    pub name: String
}

impl CsvFile {
    pub fn new(name: &str) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            name: name.to_string()
        }
    }
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct DocumentHeader {
    pub csv_file_uuid: String,
    pub header_name: String,
    pub column_idx: u16
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PdfRow {
    pub uuid: String,
    pub pdf_url: String,
    pub verified: bool
}

impl PdfRow {
    pub fn new(name: &str) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            pdf_url: name.to_string(),
            verified: false
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct DocumentCell {
    pub csv_file_uuid: String,
    pub column_idx: u16,
    pub pdf_row_uuid: String,
    pub value: String,
    pub new_value: Option<String>
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentCellResponse {
    pub csv_file_uuid: String,
    pub column_name: String,
    pub pdf_row_uuid: String,
    pub value: String,
    pub new_value: Option<String>
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvTable {
    pub headers: Vec<String>,
    pub rows_info: Vec<PdfRow>,
    pub data: Vec<Vec<DocumentCellResponse>>
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifiedRow {
    pub uuid: String,
    pub verified: bool
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CellNewValueRequest {
    pub csv_file_uuid: String,
    pub column_name: String,
    pub pdf_row_uuid: String,
    pub new_value: Option<String>
}

pub struct CellNewValue {
    pub csv_file_uuid: String,
    pub column_idx: u16,
    pub pdf_row_uuid: String,
    pub new_value: Option<String>
}