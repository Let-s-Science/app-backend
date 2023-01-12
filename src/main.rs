use opentelemetry::global;
use poem::{
    endpoint::PrometheusExporter,
    listener::TcpListener,
    session::{CookieConfig, CookieSession},
    web::cookie::CookieKey,
    EndpointExt, Server,
};
use sqlx::PgPool;
use tracing_subscriber::{layer::SubscriberExt, Registry};

pub mod core;
pub mod middleware;
pub mod routes;
pub mod security;

fn init_tracer() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_collector_pipeline()
        .with_service_name("letsscience")
        .with_endpoint("http://localhost:14268/api/traces")
        .with_username("username")
        .with_password("s3cr3t")
        .with_reqwest()
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("unable to install tracing pipeline");

    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default().with(opentelemetry);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Unable to set default global subscriber");
}

// TODO: Init migrations

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();
    init_tracer();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");

    let pool = PgPool::connect(&db_url).await.unwrap();

    let secret = std::env::var("SECRET").expect("SECRET is required");
    let cookie_config =
        CookieConfig::signed(CookieKey::from(secret.as_bytes())).name("X-SESSION-TOKEN");
    let session = CookieSession::new(cookie_config);

    let app = routes::routes()
        .at("/metrics", PrometheusExporter::new())
        .data(pool)
        .with(session)
        .with(middleware::LogMiddleware);

    Server::new(TcpListener::bind("0.0.0.0:3001"))
        .run(app)
        .await
}