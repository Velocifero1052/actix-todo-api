use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)]
pub struct Status{
    pub status: String
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonDto{
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "todo_list")]
pub struct TodoList {
    pub id: i32,
    pub title: String,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "todo_item")]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub checked: bool,
    pub list_id: i32
}