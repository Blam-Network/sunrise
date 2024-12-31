mod routes;

use axum::Router;
use axum::routing::get;
use crate::features::common::title_server::APIFeature;
use crate::features::lsp::fileshare::routes::get_files_catalog::get_files_catalog;

pub struct FileShareAPI {

}

impl APIFeature for FileShareAPI {
    fn get_router(&self) -> Router {
        Router::new()
            .route("/gameapi/FilesGetCatalog.ashx", get(get_files_catalog))
    }

    fn get_name(&self) -> &str {
        "LSP - File Share"
    }
}

