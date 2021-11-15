use crate::db::Conn;
use crate::todo::model::*;
use rocket::form::Form;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/")]
async fn index(conn: Conn) -> Template {
    let todos = Todo::all(&conn).await.unwrap_or_default();
    let mut context = HashMap::new();
    context.insert("todos", todos);
    Template::render("index", context)
}

#[post("/", data = "<todo>")]
async fn add(todo: Form<NewTodo>, conn: Conn) -> Redirect {
    match Todo::insert(todo.into_inner().title, &conn).await {
        Ok(_) => Redirect::found("/todos"),
        Err(e) => Redirect::found("/"),
    }
}

pub fn routes() -> std::vec::Vec<rocket::Route> {
    routes![index, add]
}
