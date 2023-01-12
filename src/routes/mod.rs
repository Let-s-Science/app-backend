use poem::Route;
use poem_openapi::{OpenApiService, Tags};

pub mod auth;
pub mod quiz;

#[derive(Tags)]
enum ApiTags {
    User,
}

pub fn routes() -> Route {
    let openapi_service = OpenApiService::new((auth::AuthApi), "Let's Science API", "0.1")
        .server("http://localhost:3001/api");
    let ui = openapi_service.rapidoc();
    Route::new().nest("/api", openapi_service).nest("/", ui)
}
