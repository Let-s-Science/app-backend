use poem::{endpoint::StaticFilesEndpoint, Route};
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
    let openapi_service = OpenApiService::new(
        (auth::AuthAPI, quiz::QuizAPI, challenge::ChallengeAPI),
        "Let's Science API",
        "0.1",
    )
    .server("http://localhost:3000/api");
    let docs = openapi_service.redoc();
    let files = StaticFilesEndpoint::new("./dist").index_file("index.html");
    Route::new()
        .nest_no_strip("/api", openapi_service)
        .nest("/docs", docs)
        .nest("/", files)
}
