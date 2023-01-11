use opentelemetry::global;
use poem::{endpoint::PrometheusExporter, listener::TcpListener, EndpointExt, Server};
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
        .with_hyper()
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
    init_tracer();

    let pool = PgPool::connect("postgres://letsscience:strong_password@127.0.0.1:5432/letsscience")
        .await
        .unwrap();

    let app = routes::routes()
        .at("/metrics", PrometheusExporter::new())
        .data(pool)
        .with(middleware::LogMiddleware);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
