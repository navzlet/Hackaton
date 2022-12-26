
use sqlx::{MySql, Pool};
use sqlx::mysql::MySqlPoolOptions;
use crate::models::{CsvFile, DocumentCell, DocumentHeader, CellNewValue, PdfRow, VerifiedRow};
use crate::StdErr;

#[derive(Clone)]
pub struct Db {
    pub(crate) pool: Pool<MySql>,
}

impl Db {
    pub async fn connect() -> Result<Self, StdErr> {
        let db_url = std::env::var("DATABASE_URL")?;
        println!("{}", db_url);
        let pool = MySqlPoolOptions::new()
            .max_connections(100)
            .min_connections(50)
            .connect(&db_url)
            .await?;
        Ok(Db { pool })
    }

    pub async fn list_csv_files(&self) -> Result<Vec<CsvFile>, StdErr> {
        let list_csv_files = sqlx::query_as("SELECT * FROM csv_files")
            .fetch_all(&self.pool)
            .await?;
        Ok(list_csv_files)
    }

    pub async fn exist_file_csv(&self, csv_name: &str) -> Result<bool, StdErr> {
        let exist: Vec<CsvFile>  = sqlx::query_as("SELECT * FROM csv_files WHERE name = ?;")
            .bind(csv_name)
            .fetch_all(&self.pool)
            .await?;
        Ok(!exist.is_empty())
    }

    pub async fn save_file_csv(&self, csv_file: &CsvFile) -> Result<(), StdErr> {
        sqlx::query("INSERT INTO csv_files (uuid, name) VALUES (?, ?)")
            .bind(&csv_file.uuid)
            .bind(&csv_file.name)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_file_csv_by_name(&self, name: &str) -> Result<CsvFile, StdErr>  {
        let csv_file = sqlx::query_as("SELECT * FROM csv_files WHERE name = ?")
            .bind(name)
            .fetch_one(&self.pool)
            .await?;
        Ok(csv_file)
    }

    pub async fn save_document_header(&self, header: &DocumentHeader) -> Result<(), StdErr> {
        sqlx::query("INSERT INTO documents_headers (csv_file_uuid, header_name, column_idx) VALUES(?, ?, ?);")
            .bind(&header.csv_file_uuid)
            .bind(&header.header_name)
            .bind(&header.column_idx)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_document_header(&self, column_name: &str, csv_file_uuid: &str) -> Result<DocumentHeader, StdErr> {
        let header = sqlx::query_as("SELECT * FROM documents_headers WHERE csv_file_uuid = ? AND header_name = ?")
            .bind(csv_file_uuid)
            .bind(column_name)
            .fetch_one(&self.pool)
            .await?;
        Ok(header)
    }

    pub async fn save_pdf_row(&self, pdf_row: &PdfRow) -> Result<(), StdErr> {
        sqlx::query("INSERT INTO pdfs_rows (uuid, pdf_url) VALUES(?, ?);")
            .bind(&pdf_row.uuid)
            .bind(&pdf_row.pdf_url)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn save_document_cell(&self, document_cell: &DocumentCell) -> Result<(), StdErr> {
        sqlx::query("INSERT INTO documents_cells (csv_file_uuid, column_idx, pdf_row_uuid, value) VALUES(?, ?, ?, ?);")
            .bind(&document_cell.csv_file_uuid)
            .bind(&document_cell.column_idx)
            .bind(&document_cell.pdf_row_uuid)
            .bind(&document_cell.value)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_document_headers(&self, csv_file_uuid: &str) -> Result<Vec<DocumentHeader>, StdErr> {
        let headers = sqlx::query_as("SELECT * FROM documents_headers WHERE csv_file_uuid = ? ORDER BY column_idx;")
            .bind(csv_file_uuid)
            .fetch_all(&self.pool)
            .await?;
        Ok(headers)
    }

    pub async fn get_document_cells(&self, csv_file_uuid: &str) -> Result<Vec<DocumentCell>, StdErr> {
        let cells = sqlx::query_as("SELECT * FROM documents_cells WHERE csv_file_uuid = ?")
            .bind(csv_file_uuid)
            .fetch_all(&self.pool)
            .await?;
        Ok(cells)
    }

    pub async fn get_pdf_row(&self, pdf_row_uuid: &str) -> Result<PdfRow, StdErr> {
        let row = sqlx::query_as("SELECT * FROM pdfs_rows WHERE uuid = ?")
            .bind(pdf_row_uuid)
            .fetch_one(&self.pool)
            .await?;
        Ok(row)
    }

    pub async fn update_verified_row(&self, verified_row: &VerifiedRow) -> Result<(), StdErr> {
        sqlx::query("UPDATE pdfs_rows SET verified=? WHERE uuid=?;")
            .bind(&verified_row.verified)
            .bind(&verified_row.uuid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_cell_new_value(&self, cell_new_value: &CellNewValue) -> Result<(), StdErr> {
        sqlx::query("UPDATE documents_cells SET new_value= ? WHERE csv_file_uuid=? AND column_idx=? AND pdf_row_uuid=?;")
            .bind(&cell_new_value.new_value)
            .bind(&cell_new_value.csv_file_uuid)
            .bind(&cell_new_value.column_idx)
            .bind(&cell_new_value.pdf_row_uuid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}