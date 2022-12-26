CREATE TABLE `documents_cells` (
  `csv_file_uuid` varchar(36) NOT NULL,
  `column_idx` smallint(5) unsigned NOT NULL,
  `pdf_row_uuid` varchar(36) NOT NULL,
  `value` text DEFAULT NULL,
  `new_value` text DEFAULT NULL,
  UNIQUE KEY `documents_cells_UN` (`csv_file_uuid`,`column_idx`,`pdf_row_uuid`),
  KEY `documents_cells_FK` (`pdf_row_uuid`),
  CONSTRAINT `documents_cells_FK` FOREIGN KEY (`pdf_row_uuid`) REFERENCES `pdfs_rows` (`uuid`) ON DELETE CASCADE,
  CONSTRAINT `documents_cells_FK_1` FOREIGN KEY (`csv_file_uuid`, `column_idx`) REFERENCES `documents_headers` (`csv_file_uuid`, `column_idx`) ON DELETE CASCADE
)
