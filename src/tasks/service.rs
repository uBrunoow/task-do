pub async fn get_tasks(conn: DbConn) -> Json<Vec<Task>> {
    conn.run(|c| {
        repository::get_all_tasks(c).map(Json).expect("Error loading tasks")
    }).await
}

pub async fn get_task_by_id(conn: DbConn, task_id: i32) -> Json<Task> {
    conn.run(move |c| {
        repository::get_task_by_id(c, task_id).map(Json).expect("Error loading task")
    }).await
}

pub async fn create_task(conn: DbConn, task: NewTask) -> Json<Task> {
    conn.run(move |c| {
        repository::create_task(c, task).map(Json).expect("Error creating task")
    }).await
}

pub async fn update_task(conn: DbConn, task_id: i32, task: UpdateTask) -> Json<Task> {
    conn.run(move |c| {
        repository::update_task(c, task_id, task).map(Json).expect("Error updating task")
    }).await
}

pub async fn delete_task(conn: DbConn, task_id: i32) -> Json<usize> {
    conn.run(move |c| {
        repository::delete_task(c, task_id).map(Json).expect("Error deleting task")
    }).await
}