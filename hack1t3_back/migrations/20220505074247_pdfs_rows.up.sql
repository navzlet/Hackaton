CREATE TABLE `pdfs_rows` (
  `uuid` varchar(36) NOT NULL,
  `pdf_url` varchar(255) NOT NULL,
  `verified` BOOL NOT NULL DEFAULT false,
  PRIMARY KEY (`uuid`)
)
