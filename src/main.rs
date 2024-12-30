mod features;

use std::convert::Infallible;
use std::net::SocketAddr;
use axum::{
    Router,
    routing::get,
    response::IntoResponse,
};
use axum::body::HttpBody;
use axum::extract::Request;
use dotenv::dotenv;
use hyper::body::Incoming;
use crate::features::blam_network::BlamNetwork;
use crate::features::common::database::migrations::run_migrations;
use crate::features::common::title_server::APIFeature;
use crate::features::lsp::ttl::TitleStorageServer;
#[cfg(feature = "sunrise_private")]
use sunrise_private::features::lsp::web::WebstatsServer;
#[cfg(feature = "sunrise_private")]
use sunrise_private::features::PrivateAPIFeature;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use crate::features::lsp::usr::UserStorageServer;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server;
use tower::{Service, ServiceExt};
use crate::features::lsp::gameapi_reach::GameAPIReach;
use crate::features::lsp::reach_presence_api::ReachPresenceAPI;

pub fn get_api_features() -> Vec<Box<dyn APIFeature>> {
    let mut vector = Vec::<Box<dyn APIFeature>>::new();
    vector.push(Box::new(TitleStorageServer {}));
    vector.push(Box::new(BlamNetwork {}));
    vector.push(Box::new(UserStorageServer {}));
    vector.push(Box::new(GameAPIReach {}));
    vector.push(Box::new(ReachPresenceAPI {}));
    vector
}

#[cfg(feature = "sunrise_private")]
pub fn get_private_api_features() -> Vec<Box<dyn PrivateAPIFeature>> {
    let mut vector = Vec::<Box<dyn PrivateAPIFeature>>::new();
    vector.push(Box::new(WebstatsServer {}));
    vector
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    run_migrations().await.unwrap();
    println!("Migrations applied successfully.");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let mut app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    let features = get_api_features();

    features.iter().for_each(|server| {
        app = app.clone().merge(server.get_router());
        println!("Activated Feature {}", server.get_name());
    });

    #[cfg(feature = "sunrise_private")]
    {
        let private_features = get_private_api_features();

        private_features.iter().for_each(|server| {
            app = app.clone().merge(server.get_router());
            println!("Activated Private Feature {}", server.get_name());
        });
    }

    app = app
        .layer(TraceLayer::new_for_http());

    let mut make_service = app.into_make_service_with_connect_info::<SocketAddr>();

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    loop {
        let (socket, remote_addr) = listener.accept().await.unwrap();

        let tower_service = unwrap_infallible(make_service.call(remote_addr).await);

        tokio::spawn(async move {
            let socket = TokioIo::new(socket);

            let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
                tower_service.clone().oneshot(request)
            });

            if let Err(err) = server::conn::auto::Builder::new(TokioExecutor::new())
                .http1()
                .preserve_header_case(true)
                .title_case_headers(true)
                .serve_connection(socket, hyper_service)
                .await
            {
                eprintln!("failed to serve connection: {err:#}");
            }
        });
    }
}

fn unwrap_infallible<T>(result: Result<T, Infallible>) -> T {
    match result {
        Ok(value) => value,
        Err(err) => match err {},
    }
}