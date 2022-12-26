
Instruction:

# Run
Requires: MySQL (MariaDB)

The `.env` file contains the data for processing csv files and the data for connecting to the database, which must first be modified to suit your needs.

## Linux
1. Set `DATABASE_URL` in `.env` file
2. Open terminal in the current folder
3. execute ``./hack1t3_back`` to run server
4. execute ``curl localhost:8080/api/load_csv`` to process csv files and then save them to the database

# REST API

Server port **8080**

`GET /api/load_csv` - load csv files to database

`GET /api/get_csv_files_list` - get all csv names files

`GET /api/get_table/{csv_uuid}` - get table parsed csv
**{csv_uuid}** - uuid csv file

`POST api/update_verified_row` - update verified status row
```json
{
  uuid: string,
  verified: boolean
}
```

`POST /api/update_cell_new_value`
```json
{
  csvFileUuid: string, 
  columnName: string, 
  pdfRowUuid: string, 
  newValue:  null | string
}
```