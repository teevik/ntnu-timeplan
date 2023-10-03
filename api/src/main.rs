use axum::routing::get;
use ntnu_timeplan_api::calendar::calendar_handler::calendar_handler;
use ntnu_timeplan_api::router::rspc_router;
use ntnu_timeplan_api::AppState;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let reqwest_client = reqwest::Client::new();
    let router = Arc::new(rspc_router());

    let app_state = AppState::new(&reqwest_client).await?;

    //     let app = Route::new()
    //         .nest("/", ui)
    //         .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
    //         .nest("/api", api_service.with(Cors::new()).with(Tracing))
    //         .at("/calendar.ics", get(calendar_handler).with(Tracing))
    //         .data(app_state);

    let app = axum::Router::new()
        .route(
            "/calendar.ics",
            get(calendar_handler).with_state(app_state.clone()),
        )
        .nest("/rspc", router.endpoint(move || app_state.clone()).axum())
        .layer(CorsLayer::permissive());

    let port = match env::var("PORT") {
        Ok(val) => val.parse::<u16>()?,
        Err(_) => 8080,
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::info!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
