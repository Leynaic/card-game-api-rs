mod controller;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer, web};
    use actix_cors::Cors;

    dotenv().ok(); // Load .env file

    let api_host = env::var("HOST")
        .expect("HOST must be set");

    let api_port = env::var("PORT")
        .expect("PORT must be set");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin();

        App::new()
            .wrap(cors)
            .route("/cards", web::post().to(controller::post_cards))
            .route("/cards/{id}", web::get().to(controller::get_cards))
            .route("/cards/{id}/shuffle", web::post().to(controller::shuffle_cards))
            .route("/cards/{id}/take", web::post().to(controller::take_cards))
            .route("/cards/{id}/put", web::post().to(controller::put_cards))
            .route("/cards/{id}", web::delete().to(controller::delete_cards))
    })
        .bind(&[ api_host, api_port ].join(":"))?
        .run()
        .await
}
