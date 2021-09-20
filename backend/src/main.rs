mod api;
#[macro_use] extern crate rocket;

use {
    rocket::{routes, get},
    rocket::fs::{NamedFile, FileServer, relative},
    std::path::PathBuf,
};


#[get("/createtable")]
async fn index() -> Option<NamedFile> {
    let default_dir :&str = concat!(env!("CARGO_MANIFEST_DIR"), "/static");
    let mut path = PathBuf::from(default_dir);
    path.push("index.html");
    NamedFile::open(path).await.ok()
}




#[launch]
fn rocket() -> _ {
    let routes = routes![index];
    rocket::build()
        .mount("/", routes)
        .mount("/", FileServer::from(relative!("/static")))
        .mount("/api", api::get_api_routes())
}

