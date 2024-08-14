use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::schema::tasks;

#[derive(Queryable, Serialize, Deserialize)]
#[table_name = "tasks"]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<chrono::NaiveDate>,
    pub priority_id: Option<i32>,
    pub tags_id: Option<i32>,
    pub created_by: Option<i32>,
    pub assigned_to: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Queryable, Serialize, Deserialize)]
#[table_name = "tasks"]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub due_date: Option<chrono::NaiveDate>,
    pub priority_id: Option<i32>,
    pub tags_id: Option<i32>,
    pub created_by: Option<i32>,
    pub assigned_to: Option<i32>,
}

#[derive(Queryable, Serialize, Deserialize)]
#[table_name = "tasks"]
pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<chrono::NaiveDate>,
    pub priority_id: Option<i32>,
    pub tags_id: Option<i32>,
    pub created_by: Option<i32>,
    pub assigned_to: Option<i32>,
}