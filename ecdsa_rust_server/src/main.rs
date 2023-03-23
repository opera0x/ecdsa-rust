mod routes;

use routes::create_routes;

#[tokio::main]
async fn main() {
    let app = create_routes();

    axum::Server::bind(&"0.0.0.0:3042".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
