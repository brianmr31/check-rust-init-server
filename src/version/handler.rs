use crate::connection;
use crate::core::core::ApiResponse;
use rocket::http::Status;

#[get("/")]
pub fn index(_connection: connection::DbConn) -> ApiResponse {
    ApiResponse {
        json: json!({"version": "v1.0.0", "server": "init-server"}),
        status: Status::Ok,
    }
}
