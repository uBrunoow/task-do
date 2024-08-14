#[get("/tasks")]
pub async fn get_tasks(_auth: AuthGuard, conn: DbConn) -> Json<Vec<Tasks>> {
    service::get_tasks(conn).await
}

pub fn get_routes() -> Vec<Route> {
  routes![get_tasks]
}