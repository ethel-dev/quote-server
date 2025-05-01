use std::net::SocketAddr;

mod routes;
mod models;

#[tokio::main]
async fn main() {
    let quote = models::Quote {
        text: String::from("The only limit to our realization of tomorrow is our doubts of today."),
        author: String::from("Franklin D. Roosevelt"),
    };

    let app = routes::app().with_state(quote);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}