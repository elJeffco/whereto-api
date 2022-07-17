use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse {
    message: String,
}

#[catch(404)]
pub fn internal_404() -> Json<ApiResponse> {
    let message = ApiResponse {
        message: "The requested url was not found on the server.".to_string(),
    };

    Json(message)
}

#[catch(500)]
pub fn internal_500() -> Json<ApiResponse> {
    let message = ApiResponse {
        message: "The server encountered an error and could not complete your request.".to_string(),
    };

    Json(message)
}
