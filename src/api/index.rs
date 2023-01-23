use actix_files::NamedFile;
use actix_web::{get, HttpRequest, Result};
use std::path::PathBuf;


#[get("/")]
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open(PathBuf::from("./dist/index.html"))?)
}
