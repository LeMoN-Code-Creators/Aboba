#[macro_use] extern crate rocket;

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use rocket::form::error::Entity::Name;
use rocket::fs::NamedFile;
use rocket::response::Redirect;
use rocket::response::status::NotFound;


#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile>{
    let extension = file.extension();
    let extension = if let Some(res) = extension{
        res
    }else {
        OsStr::new("")
    };
    match extension.to_str().unwrap() {
        "html" => NamedFile::open(Path::new("./static/html/").join(file)).await.ok(),
        "css"  => NamedFile::open(Path::new("./static/css/").join(file)).await.ok(),
        "js"   => NamedFile::open(Path::new("./static/js/").join(file)).await.ok(),
        "ico"|"png"|"jpg"|"svg" => NamedFile::open(Path::new("./static/img/").join(file)).await.ok(),
        _      => NamedFile::open("/static/404.html").await.ok()
    }
}


#[get("/")]
fn index() -> Redirect {
    Redirect::to("/main.html")
}
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![files])
        .launch()
        .await?;
    Ok(())
}