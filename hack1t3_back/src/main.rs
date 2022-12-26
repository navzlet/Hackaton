mod cotrollers;
mod db;
mod models;
mod utils;
mod logger;


use std::sync::Mutex;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_cors::Cors;
use actix_web::web::{Data};


use crate::cotrollers::api;
use crate::db::Db;
use crate::models::CsvFile;

type StdErr = Box<dyn std::error::Error>;

#[actix_web::get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> Result<(), StdErr> {
    dotenv::dotenv().ok();
    #[cfg(debug_assertions)]
    logger::init()?;

    let data = db::Db::connect().await?;

    sqlx::migrate!()
        .run(&data.pool)
        .await?;

    HttpServer::new(move || {
        let cors = Cors::default()
            .send_wildcard()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(None);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(data.clone()))
            .service(hello)
            .service(api())
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await?;

    Ok(())
}
