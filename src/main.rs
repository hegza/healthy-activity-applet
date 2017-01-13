#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use]
extern crate log;

extern crate url;
extern crate time;

// For reading files
use std::path::{Path,PathBuf};
use rocket::response::NamedFile;

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("./views/index.html")).ok()
}

#[get("/<file..>")]
fn get_node_modules(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("./node_modules/").join(file)).ok()
}

#[get("/<file..>")]
fn get_dist(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("./dist/").join(file)).ok()
}

fn main() {
    rocket::ignite()
            .mount("/", routes![index])
            .mount("/node_modules", routes![get_node_modules])
            .mount("/dist", routes![get_dist])
            .launch();
}
