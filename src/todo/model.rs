use crate::db::Conn;
use crate::schema::todos;
use crate::schema::todos::dsl::todos as all_todos;
use chrono::{NaiveDateTime, Utc};
use diesel::result::Error;
use diesel::{self, prelude::*, result::QueryResult};
use diesel::{Insertable, Queryable};
use rocket::serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, FromForm, Insertable)]
#[table_name = "todos"]
pub struct NewTodo {
    pub title: String,
}

#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub created_timestamp: NaiveDateTime,
}

impl Todo {
    pub async fn all(conn: &Conn) -> QueryResult<Vec<Todo>> {
        conn.run(|c| {
            all_todos
                .order(todos::created_timestamp.desc())
                .load::<Todo>(c)
        })
        .await
    }

    pub async fn insert(title: String, conn: &Conn) -> Result<Todo, Error> {
        conn.run(|c| {
            let t = Todo {
                id: Uuid::new_v4(),
                title: title,
                completed: false,
                created_timestamp: Utc::now().naive_utc(),
            };
            match diesel::insert_into(todos::table).values(&t).execute(c) {
                Ok(_) => Result::Ok(t),
                Err(e) => Result::Err(e),
            }
        })
        .await
    }

    pub async fn delete(id: Uuid, conn: &Conn) -> Result<(), Error> {
        conn.run(
            move |c| match diesel::delete(todos::table.find(id)).execute(c) {
                Ok(_) => Result::Ok(()),
                Err(e) => Result::Err(e),
            },
        )
        .await
    }
}
