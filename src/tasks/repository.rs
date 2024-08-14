use diesel::prelude::*;
use crate::tasks::models::tasks::{Task, NewTask, UpdateTask};

pub fn get_all_tasks(conn: &mut PgConnection) -> QueryResult<Vec<Task>> {
    tasks::table.load::<Task>(conn)
}

pub fn get_task_by_id(conn: &mut PgConnection, task_id: i32) -> QueryResult<Task> {
    tasks::table.find(task_id).first::<Task>(conn)
}

pub fn create_task(conn: &mut PgConnection, new_task: NewTask) -> QueryResult<Task> {
    diesel::insert_into(tasks::table)
        .values(&new_task)
        .get_result(conn)
}

pub fn update_task(conn: &mut PgConnection, task_id: i32, updated_task: UpdateTask) -> QueryResult<Task> {
    diesel::update(tasks::table.find(task_id))
        .set(&updated_task)
        .get_result(conn)
}

pub fn delete_task(conn: &mut PgConnection, task_id: i32) -> QueryResult<usize> {
    diesel::delete(tasks::table.find(task_id))
        .execute(conn)
}