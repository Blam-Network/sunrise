mod features;

use axum::{
    routing::get,
    Router,
};
use crate::features::blam_network::BlamNetwork;
use crate::features::common::database::migrations::run_migrations;
use crate::features::common::title_server::APIFeature;
use crate::features::lsp::ttl::TitleStorageServer;
#[cfg(feature = "sunrise_private")]
use sunrise_private::features::lsp::web::WebstatsServer;
#[cfg(feature = "sunrise_private")]
use sunrise_private::features::PrivateAPIFeature;

pub fn get_api_features() -> Vec<Box<dyn APIFeature>> {
    let mut vector = Vec::<Box<dyn APIFeature>>::new();
    vector.push(Box::new(TitleStorageServer {}));
    vector.push(Box::new(BlamNetwork {}));
    vector
}

#[cfg(feature = "sunrise_private")]
pub fn get_private_api_features() -> Vec<Box<dyn PrivateAPIFeature>> {
    let mut vector = Vec::<Box<dyn APIFeature>>::new();
    vector.push(Box::new(WebstatsServer {}));
    vector
}

#[tokio::main]
async fn main() {
    run_migrations().await.unwrap();
    println!("Migrations applied successfully.");

    let mut app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let features = get_api_features();

    features.iter().for_each(|server| {
        app = app.clone().merge(server.get_router());
    });

    #[cfg(feature = "sunrise_private")]
    {
        let private_features = get_private_api_features();

        private_features.iter().for_each(|server| {
            app = app.clone().merge(server.get_router());
        });
    }

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}