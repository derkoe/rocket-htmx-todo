use rocket_dyn_templates::Template;
use std::collections::HashMap;
use rocket::serde::uuid::Uuid;
use serde_json::value::to_value;
use rocket::form::Form;
use rocket::serde::Serialize;
use rocket::response::Redirect;

#[derive(Serialize, Debug, FromForm)]
pub struct Todo {
    pub id: Option<Uuid>,
    pub title: String,
    pub completed: bool,
}

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("todos", to_value(&vec![Todo {
        id: Option::from(Uuid::default()),
        title: "Hello".to_string(),
        completed: false,
    }]).unwrap());
    Template::render("index",  context)
}


#[post("/", data = "<todo>")]
fn add(todo: Form<Todo>) -> Redirect {
    println!("Add todo: {}", todo.title);
    Redirect::found("/todos")
}

pub fn routes() -> std::vec::Vec<rocket::Route> {
    routes![index, add]
}
