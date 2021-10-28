mod controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer, web};

    HttpServer::new(|| {
        App::new()
            .route("/cards", web::post().to(controller::post_cards))
            .route("/cards/{id}", web::get().to(controller::get_cards))
            .route("/cards/{id}/shuffle", web::post().to(controller::shuffle_cards))
            .route("/cards/{id}/take", web::post().to(controller::take_cards))
            .route("/cards/{id}/put", web::post().to(controller::put_cards))
            .route("/cards/{id}", web::delete().to(controller::delete_cards))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
