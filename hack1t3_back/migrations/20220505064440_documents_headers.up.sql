CREATE TABLE `documents_headers` (
  `csv_file_uuid` varchar(36) NOT NULL,
  `header_name` varchar(255) NOT NULL,
  `column_idx` smallint(5) unsigned NOT NULL,
  UNIQUE KEY `documents_headers_UN` (`csv_file_uuid`,`column_idx`),
  CONSTRAINT `documents_headers_FK` FOREIGN KEY (`csv_file_uuid`) REFERENCES `csv_files` (`uuid`) ON DELETE CASCADE
)
