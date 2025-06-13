mod user;

use axum::Router;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::context::{AppApiCtx, AppCtx};

pub fn router(ctx: AppCtx) -> Router {
    let config = ctx.config.clone();
    let api_prefix = config
        .server
        .prefix
        .as_ref()
        .map(String::as_str)
        .unwrap_or("/");

    let api_router = api_router(ctx.api);

    if api_prefix == "/" {
        Router::new()
            .merge(api_router)
            .layer(TraceLayer::new_for_http())
            .layer(CorsLayer::new())
    } else {
        Router::new()
            .nest(api_prefix, api_router)
            .layer(TraceLayer::new_for_http())
            .layer(CorsLayer::new())
    }
}

fn api_router(apis: AppApiCtx) -> Router {
    Router::new().nest("/users", user::router(apis.user))
}
