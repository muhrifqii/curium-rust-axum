use main::{context::AppCtx, router};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let ctx = AppCtx::init().await.unwrap();
    let server_url = ctx.config.server.url();

    let app_router = router::router(ctx);
    let listener = tokio::net::TcpListener::bind(server_url).await.unwrap();
    axum::serve(listener, app_router).await.unwrap();
}
