use actix_web::{web};
use actix_web::dev::HttpServiceFactory;
use crate::cotrollers::get_table_controller::get_table;
use crate::cotrollers::loader_controller::load_scv;
use crate::cotrollers::svg_files_contoller::{exist_csv_file, get_csv_files_list};
use crate::cotrollers::update_table_data_controller::{update_cell_new_value, update_verified_row};

mod svg_files_contoller;
mod loader_controller;
mod get_table_controller;
mod update_table_data_controller;

pub fn api() -> impl HttpServiceFactory + 'static {
    web::scope("/api")
        .service(load_scv)
        .service(exist_csv_file)
        .service(get_csv_files_list)
        .service(update_verified_row)
        .service(update_cell_new_value)
        .service(get_table)
}