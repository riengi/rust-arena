/* Rocket.rs (0.5) backend:
1. route (request)
2. validate (request)
3. process (business, produce Response)
4. response (HTTP response generation and sending)
*/

use std::path::{Path, PathBuf};

use rocket::fs::{FileServer, NamedFile};

#[macro_use]
extern crate rocket;

// Root path
#[get("/")]
fn index() -> &'static str {
    "Index"
}

// dynamic path
#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("hello {}", name)
}

// multiple segments
#[get("/hello-title/<title>/<surname>")]
fn hello_title(title: &str, surname: &str) -> String {
    format!("hello {} {}", title, surname)
}

// fixed path
#[get("/hello-world")]
fn hello_world() -> &'static str {
    "Hello World!"
}

// get file content (safely)
#[get("/file/<f..>")]
async fn file(f: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(f)).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![hello])
        .mount("/", routes![hello_world])
        .mount("/", routes![hello_title])
        .mount("/", routes![file])
        // easy static content serving
        .mount("/public", FileServer::from("static/"))
}
