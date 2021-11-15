use crate::db::Conn;
use crate::todo::model::*;
use rocket::form::Form;
use rocket::http::Status;
use rocket::request::FromParam;
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

#[get("/")]
async fn index(conn: Conn) -> Template {
    let todos = Todo::all(&conn).await.unwrap_or_default();
    let mut context = HashMap::new();
    context.insert("todos", todos);
    Template::render("index", context)
}

#[derive(Debug, Responder)]
pub enum TemplateOrRedirect {
    Template(Template),
    Redirect(Redirect),
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
        Err(e) => Result::Err(Status::default()),
    }
}

#[post("/<id>/delete")]
async fn delete(id: Uuid, conn: Conn) -> Result<Redirect, Status> {
    match Todo::delete(id, &conn).await {
        Ok(_) => Result::Ok(Redirect::to(uri!("/todos"))),
        Err(e) => Result::Err(Status::default()),
    }
}

pub fn routes() -> std::vec::Vec<rocket::Route> {
    routes![index, add]
}
