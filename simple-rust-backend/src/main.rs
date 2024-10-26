use axum::{body::Body, http::{Method, Request}, middleware::{self, Next}, response::Response, routing::get, Router};

async fn logging_middleware(req: Request<Body>, next: Next) -> Response {
    println!("Received a request to {}", req.uri());
    next.run(req).await
}
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new().route("/api/endpoint", get(|| async { "Hello, Rust!" }))
    .layer(middleware::from_fn(logging_middleware))
    .layer(cors);

    println!("Running on http://0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
    .await
    .unwrap();

    axum::serve(listener,app).await.unwrap();

}
