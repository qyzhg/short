use actix_files as fs;
use std::path::PathBuf;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, http, HttpRequest, Result};
use actix_web::http::StatusCode;
use actix_web::web::service;
use log::{LevelFilter, info};
use settings::Settings;
use simple_logger::SimpleLogger;
use sqlx::mysql::MySqlPoolOptions;
use actix_cors::Cors;


mod api;
mod settings;

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    let s = Settings::new().unwrap();
    let ip = s.server.get_ip();
    let url = s.database.url;
    let pool_size = s.database.pool_size;

    let pool = MySqlPoolOptions::new()
        .max_connections(pool_size)
        .connect(&url)
        .await?;

    info!("server listening at http://{:?}", &ip);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .service(api::index::index)
            .service(fs::Files::new("/static", "./dist/static").show_files_listing())
            .service(fs::Files::new("/css", "./dist/css").show_files_listing())
            .service(fs::Files::new("/js", "./dist/js").show_files_listing())
            .service(api::links::create_link)
            .service(api::links::get_all_links)
            .service(api::links::get_from_link)
            .service(api::links::get_origin_url_from_link)
    })
    // .bind(("127.0.0.1", 8080))?
    .bind(&ip)?
    .run()
    .await?;

    Ok(())
}
