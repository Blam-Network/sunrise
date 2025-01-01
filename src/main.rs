mod features;

use std::convert::Infallible;
use std::{env, fs, io};
use std::net::SocketAddr;
use std::sync::Arc;
use axum::{
    routing::get,
    Router,
};
use axum::extract::connect_info::IntoMakeServiceWithConnectInfo;
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
use rustls::ServerConfig;
use tower::{Service, ServiceExt};
use tracing::log::error;
use crate::features::lsp::fileshare::FileShareAPI;
use crate::features::lsp::gameapi_reach::GameAPIReach;
use crate::features::lsp::reach_presence_api::ReachPresenceAPI;
use rustls_pki_types::{CertificateDer, PrivateKeyDer};
use tokio::sync::Mutex;
use tokio_rustls::TlsAcceptor;

pub fn get_api_features() -> Vec<Box<dyn APIFeature>> {
    let mut vector = Vec::<Box<dyn APIFeature>>::new();
    vector.push(Box::new(TitleStorageServer {}));
    vector.push(Box::new(BlamNetwork {}));
    vector.push(Box::new(UserStorageServer {}));
    vector.push(Box::new(GameAPIReach {}));
    vector.push(Box::new(ReachPresenceAPI {}));
    vector.push(Box::new(FileShareAPI {}));
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

    let use_https = env::var("USE_HTTPS").unwrap_or(String::from("false")) == "true";
    let http_port = env::var("HTTP_PORT").unwrap_or(String::from("8080"));
    let https_port = env::var("HTTPS_PORT").unwrap_or(String::from("443"));
    let private_key_path = env::var("SSL_PRIVATE_KEY_PATH").unwrap_or(String::from(""));
    let cert_path = env::var("SSL_CERTIFICATE_PATH").unwrap_or(String::from(""));

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
            println!("Activated Feature {} (Private)", server.get_name());
        });
    }

    app = app
        .layer(TraceLayer::new_for_http());

    let http_service = app.clone().into_make_service_with_connect_info::<SocketAddr>();
    let http_listener = Arc::new(TcpListener::bind(format!("0.0.0.0:{}", http_port)).await.unwrap());
    println!("Listening on HTTP Port {}", http_port);


    if use_https {
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

        let certs = load_certs(cert_path).unwrap();
        let key = load_private_key(private_key_path).unwrap();

        let mut server_config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
            .unwrap();
        server_config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec(), b"http/1.0".to_vec()];
        let https_listener = Arc::new(TcpListener::bind(format!("0.0.0.0:{}", https_port)).await.unwrap());
        let tls_acceptor = TlsAcceptor::from(Arc::new(server_config));
        let https_service = app.into_make_service_with_connect_info::<SocketAddr>();

        println!("Listening on HTTPS Port {}", https_port);

        tokio::join!(
            accept_http_connection(http_listener, http_service.clone()),
            accept_https_connection(https_listener, Arc::new(tls_acceptor), https_service.clone())
        );
    }
    else {
        let http2_listener = Arc::new(TcpListener::bind(format!("0.0.0.0:{}", 8001)).await.unwrap());
        let http2_service = app.into_make_service_with_connect_info::<SocketAddr>();

        tokio::join!(
            accept_http_connection(http_listener, http_service.clone()),
            accept_http_connection(http2_listener, http2_service.clone())
        );
    }
}

fn load_certs(filename: String) -> io::Result<Vec<CertificateDer<'static>>> {
    let certfile = fs::File::open(&filename)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("failed to open {}: {}", &filename, e)))?;
    let mut reader = io::BufReader::new(certfile);

    rustls_pemfile::certs(&mut reader).collect()
}

fn load_private_key(filename: String) -> io::Result<PrivateKeyDer<'static>> {
    let keyfile = fs::File::open(&filename)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("failed to open {}: {}", &filename, e)))?;
    let mut reader = io::BufReader::new(keyfile);

    rustls_pemfile::private_key(&mut reader).map(|key| key.unwrap())
}

async fn accept_http_connection(
    listener: Arc<TcpListener>,
    mut make_service: IntoMakeServiceWithConnectInfo<Router, SocketAddr>,
) {
    loop {
        let (socket, remote_addr) = listener.accept().await.unwrap();
        let tower_service = unwrap_infallible(make_service.call(remote_addr).await);
        let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
            tower_service.clone().oneshot(request)
        });

        if let Err(err) = server::conn::auto::Builder::new(TokioExecutor::new())
            .http1()
            .preserve_header_case(true)
            .title_case_headers(true)
            .serve_connection(TokioIo::new(socket), hyper_service)
            .await
        {
            eprintln!("failed to serve connection: {err:#}");
        }
    }
}

async fn accept_https_connection(
    listener: Arc<TcpListener>,
    tls_acceptor: Arc<TlsAcceptor>,
    mut make_service: IntoMakeServiceWithConnectInfo<Router, SocketAddr>,
) {
    loop {
        let (socket, remote_addr) = listener.accept().await.unwrap();
        let tls_stream = match tls_acceptor.accept(socket).await {
            Ok(tls_stream) => tls_stream,
            Err(err) => {
                eprintln!("TLS handshake error: {err:#}");
                continue;
            }
        };
        let tower_service = unwrap_infallible(make_service.call(remote_addr).await);
        let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
            tower_service.clone().oneshot(request)
        });

        if let Err(err) = server::conn::auto::Builder::new(TokioExecutor::new())
            .http1()
            .preserve_header_case(true)
            .title_case_headers(true)
            .serve_connection(TokioIo::new(tls_stream), hyper_service)
            .await
        {
            eprintln!("failed to serve connection: {err:#}");
        }
    }
}

fn unwrap_infallible<T>(result: Result<T, Infallible>) -> T {
    match result {
        Ok(value) => value,
        Err(err) => match err {},
    }
}