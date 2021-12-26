use crate::db::Conn;
use crate::todo::model::*;
use rocket::form::Form;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::request::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use uuid::Uuid;

struct HxRequest(bool);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HxRequest {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let hx_request = request.headers().get_one("HX-Request");
        if hx_request.is_some() {
            Outcome::Success(HxRequest(true))
        } else {
            Outcome::Success(HxRequest(false))
        }
    }
}

#[derive(Debug, Responder)]
pub enum TemplateOrRedirect {
    Template(Template),
    Redirect(Redirect),
}

#[get("/")]
async fn index(conn: Conn) -> Template {
    let todos = Todo::all(&conn).await.unwrap_or_default();
    let mut context = HashMap::new();
    context.insert("todos", todos);
    // TODO howto add different items? context.insert("items_left", todos.len());
    Template::render("index", context)
}

#[post("/", data = "<todo>")]
async fn add(
    todo: Form<NewTodo>,
    hx_request: HxRequest,
    conn: Conn,
) -> Result<TemplateOrRedirect, Status> {
    match Todo::insert(todo.into_inner().title, &conn).await {
        Ok(todo) => {
            if hx_request.0 {
                let mut context = HashMap::new();
                context.insert("todo", todo);
                Result::Ok(TemplateOrRedirect::Template(Template::render(
                    "todos/item",
                    context,
                )))
            } else {
                Result::Ok(TemplateOrRedirect::Redirect(Redirect::found("/todos")))
            }
        }
        Err(_) => Result::Err(Status::default()),
    }
}

#[post("/<id>", data = "<todo>")]
async fn edit(id: Uuid, todo: Form<NewTodo>, conn: Conn) -> Result<Redirect, Status> {
    match Todo::edit(id, todo.into_inner().title, &conn).await {
        Ok(_) => Result::Ok(Redirect::to(uri!("/todos"))),
        Err(_) => Result::Err(Status::default()),
    }
}

#[post("/<id>/delete")]
async fn delete(id: Uuid, conn: Conn) -> Result<Redirect, Status> {
    match Todo::delete(id, &conn).await {
        Ok(_) => Result::Ok(Redirect::to(uri!("/todos"))),
        Err(_) => Result::Err(Status::default()),
    }
}

#[post("/<id>/toggle")]
async fn toggle(id: Uuid, conn: Conn) -> Result<Redirect, Status> {
    match Todo::toggle(id, &conn).await {
        Ok(_) => Result::Ok(Redirect::to(uri!("/todos"))),
        Err(_) => Result::Err(Status::default()),
    }
}

#[post("/toggle-all")]
fn toggle_all() -> Result<Redirect, Status> {
    Ok(Redirect::to(uri!("/todos")))
}

pub fn routes() -> std::vec::Vec<rocket::Route> {
    routes![index, add, edit, delete, toggle, toggle_all]
}
