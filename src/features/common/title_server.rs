use axum::Router;

pub trait APIFeature {
    fn get_router(&self) -> Router;
    fn get_name(&self) -> &str;
}