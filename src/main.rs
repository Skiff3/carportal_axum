use axum::routing::get;
mod cars;
mod data;
use crate::data::DATA;
use std::thread;
use crate::cars::Cars;

#[tokio::main]
pub async fn main() {
     // Build our application by creating our router.
    // let app = axum::Router::new()
    //     .route("/",
    //         get(hello)
    //     );

    let app = axum::Router::new()
        .route("/users.html",
            get(hello_html)
        )
        .route("/show.html",
            get(show_html)
        )
        .route("/cars",
            get(get_cars)
        );
    
    print_data().await;

    // Run our application as a hyper server on http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}

async fn print_data() {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        println!("data: {:?}" ,data);
    }).join().unwrap()
}

/// axum handler for "GET /books" which responds with a resource page.
/// This demo uses our DATA; a production app could use a database.
/// This demo must clone the DATA in order to sort items by title.
pub async fn get_cars() -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        let mut car = data.values().collect::<Vec<_>>().clone();
        car.sort_by(|a, b| a.name.cmp(&b.name));
        car.iter().map(|&car|
            format!("<p>{}</p>\n", &car)
        ).collect::<String>()
    }).join().unwrap().into()
}

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(
    uri: axum::http::Uri
) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("No route {}", uri))
}

/// axum handler for "GET /" which returns a string and causes axum to
/// immediately respond with status code `200 OK` and with the string.
pub async fn hello() -> String {
    "Hello, World!".to_string()
}


/// axum handler that responds with typical HTML coming from a file.
/// This uses the Rust macro `std::include_str` to include a UTF-8 file
/// path, relative to `main.rs`, as a `&'static str` at compile time.
async fn hello_html() -> axum::response::Html<&'static str> {
    include_str!("users.html").into()
}

async fn show_html() -> axum::response::Html<&'static str> {
    include_str!("show.html").into()
}

