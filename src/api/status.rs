use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct PingResponse {
    message: String,
}

#[get("/status/ping")]
pub fn ping() -> Json<PingResponse> {
    let message = PingResponse {
        message: "ping".to_string(),
    };

    Json(message)
}
