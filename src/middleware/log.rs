use poem::{async_trait, Endpoint, IntoResponse, Middleware, Request, Response, Result};
pub struct LogMiddleware;

impl<E: Endpoint> Middleware<E> for LogMiddleware {
    type Output = LogImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        LogImpl(ep)
    }
}

pub struct LogImpl<E>(E);

#[async_trait]
impl<E: Endpoint> Endpoint for LogImpl<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        tracing::info!(
            target:"access",
            "{} {} {}",
            req.method(),
            req.uri().path(),
            req.remote_addr()
        );
        Ok(self.0.call(req).await?.into_response())
    }
}
