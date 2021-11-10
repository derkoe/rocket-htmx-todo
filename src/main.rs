#[macro_use]
extern crate rocket;

use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use rocket::fs::{FileServer, relative};

mod todos;

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/todos"))
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/todos", todos::routes())
        .mount("/", FileServer::from(relative!("/static")))
}
