use axum::{Router, routing::get, response::Html};

async fn hello_world() -> Html<&'static str> {
    Html("<h1>Hello, Rust Web!</h1>")
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
