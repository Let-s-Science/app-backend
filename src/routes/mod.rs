use poem::Route;
use poem_openapi::Tags;

pub mod auth;

#[derive(Tags)]
enum ApiTags {
    User,
}

pub fn routes() -> Route {
    auth::routes()
}
