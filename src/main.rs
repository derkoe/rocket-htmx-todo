#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use self::db::Conn;
use rocket::fs::{relative, FileServer};
use rocket::response::Redirect;
use rocket_dyn_templates::Template;

mod db;
mod schema;
mod todo;

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/todos"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .attach(Conn::fairing())
        .mount("/", routes![index])
        .mount("/todos", todo::web::routes())
        .mount("/", FileServer::from(relative!("/static")))
}
