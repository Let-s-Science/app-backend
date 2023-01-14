use poem::Route;
use poem_openapi::{OpenApiService, Tags};

pub mod auth;
pub mod challenge;
pub mod quiz;

#[derive(Tags)]
enum ApiTags {
    User,
    Quiz,
    Challenge,
}

pub fn routes() -> Route {
    let openapi_service =
        OpenApiService::new((auth::AuthAPI, quiz::QuizAPI), "Let's Science API", "0.1")
            .server("http://localhost:3000/api");
    let ui = openapi_service.redoc();
    Route::new().nest("/api", openapi_service).nest("/", ui)
}
