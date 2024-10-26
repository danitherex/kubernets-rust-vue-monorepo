use axum::{body::Body, http::Request, middleware::{self, Next}, response::Response, routing::get, Router};

async fn logging_middleware(req: Request<Body>, next: Next) -> Response {
    println!("Received a request to {}", req.uri());
    next.run(req).await
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/endpoint", get(|| async { "Hello, Rust!" }))
    .layer(middleware::from_fn(logging_middleware));

    println!("Running on http://0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
    .await
    .unwrap();

    axum::serve(listener,app).await.unwrap();

}
