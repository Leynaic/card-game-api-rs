use jeu_de_carte::{Deck, DeckRepresentation, DeckSize, JsonMessage};
use actix_web::{HttpResponse, Result, HttpRequest, web};
use uuid::Uuid;
use serde::Deserialize;
use jeu_de_carte::database::establish_connection;

#[derive(Deserialize)]
pub struct DeckSizeRequest {
    pub size: Option<i32>,
}

#[derive(Deserialize)]
pub struct DeckShuffleRequest {
    pub shuffle_discarded: Option<bool>,
}

#[derive(Deserialize)]
pub struct DeckSwitch {
    pub lifo: Option<bool>,
    pub length: Option<usize>,
    pub move_as_block: Option<bool>,
}

pub async fn post_cards(query: web::Query<DeckSizeRequest>) -> Result<HttpResponse> {
    let deck_size = match query.size {
        Some(32) => DeckSize::Small,
        _ => DeckSize::Normal
    };
    let deck = Deck::new(deck_size);
    let mut connection = establish_connection();
    deck.insert_into_db(&mut connection);
    connection.close().ok();

    Ok(HttpResponse::Created().json(DeckRepresentation::from(deck)))
}

pub async fn shuffle_cards(req: HttpRequest, query: web::Query<DeckShuffleRequest>) -> Result<HttpResponse> {
    let deck_id = req.match_info().get("id");
    match deck_id {
        Some(deck_id) => {
            let deck_id = Uuid::parse_str(deck_id);
            match deck_id {
                Ok(deck_id) => {
                    let mut connection = establish_connection();
                    let deck = Deck::find_by_id(deck_id, &mut connection);
                    match deck {
                        Some(mut deck) => {
                            match query.shuffle_discarded {
                                Some(true) => deck.shuffle(true),
                                _ => deck.shuffle(false)
                            }
                            deck.update_db(&mut connection);
                            connection.close().ok();

                            Ok(HttpResponse::Ok().json(DeckRepresentation::from(deck)))
                        }
                        None => Ok(HttpResponse::NotFound().json(JsonMessage { message: "Impossible de trouver ce paquet de carte." }))
                    }
                }
                Err(_) => Ok(HttpResponse::UnprocessableEntity().json(JsonMessage { message: "Paramètre incorrect." }))
            }
        }
        None => Ok(HttpResponse::MethodNotAllowed().json(JsonMessage { message: "Paramètre manquant." }))
    }
}

pub async fn take_cards(req: HttpRequest, query: web::Query<DeckSwitch>) -> Result<HttpResponse> {
    let deck_id = req.match_info().get("id");
    match deck_id {
        Some(deck_id) => {
            let deck_id = Uuid::parse_str(deck_id);
            match deck_id {
                Ok(deck_id) => {
                    let mut connection = establish_connection();
                    let deck = Deck::find_by_id(deck_id, &mut connection);
                    match deck {
                        Some(mut deck) => {
                            deck.take(
                                query.lifo.unwrap_or(false),
                                query.length.unwrap_or(1),
                                query.move_as_block.unwrap_or(false)
                            );
                            deck.update_db(&mut connection);
                            connection.close().ok();

                            Ok(HttpResponse::Ok().json(DeckRepresentation::from(deck)))
                        }
                        None => Ok(HttpResponse::NotFound().json(JsonMessage { message: "Impossible de trouver ce paquet de carte." }))
                    }
                }
                Err(_) => Ok(HttpResponse::UnprocessableEntity().json(JsonMessage { message: "Paramètre incorrect." }))
            }
        }
        None => Ok(HttpResponse::MethodNotAllowed().json(JsonMessage { message: "Paramètre manquant." }))
    }
}

pub async fn put_cards(req: HttpRequest, query: web::Query<DeckSwitch>) -> Result<HttpResponse> {
    let deck_id = req.match_info().get("id");
    match deck_id {
        Some(deck_id) => {
            let deck_id = Uuid::parse_str(deck_id);
            match deck_id {
                Ok(deck_id) => {
                    let mut connection = establish_connection();
                    let deck = Deck::find_by_id(deck_id, &mut connection);
                    match deck {
                        Some(mut deck) => {
                            deck.put(
                                query.lifo.unwrap_or(false),
                                query.length.unwrap_or(1),
                                query.move_as_block.unwrap_or(false)
                            );
                            deck.update_db(&mut connection);
                            connection.close().ok();

                            Ok(HttpResponse::Ok().json(DeckRepresentation::from(deck)))
                        }
                        None => Ok(HttpResponse::NotFound().json(JsonMessage { message: "Impossible de trouver ce paquet de carte." }))
                    }
                }
                Err(_) => Ok(HttpResponse::UnprocessableEntity().json(JsonMessage { message: "Paramètre incorrect." }))
            }
        }
        None => Ok(HttpResponse::MethodNotAllowed().json(JsonMessage { message: "Paramètre manquant." }))
    }
}

pub async fn get_cards(req: HttpRequest) -> Result<HttpResponse> {
    let deck_id = req.match_info().get("id");
    match deck_id {
        Some(deck_id) => {
            let deck_id = Uuid::parse_str(deck_id);
            match deck_id {
                Ok(deck_id) => {
                    let mut connection = establish_connection();
                    let deck = Deck::find_by_id(deck_id, &mut connection);
                    connection.close().ok();

                    match deck {
                        Some(deck) => Ok(HttpResponse::Ok().json(DeckRepresentation::from(deck))),
                        None => Ok(HttpResponse::NotFound().json(JsonMessage { message: "Impossible de trouver ce paquet de carte." }))
                    }
                }
                Err(_) => Ok(HttpResponse::UnprocessableEntity().json(JsonMessage { message: "Paramètre incorrect." }))
            }
        }
        None => Ok(HttpResponse::MethodNotAllowed().json(JsonMessage { message: "Paramètre manquant." }))
    }
}

pub async fn delete_cards(req: HttpRequest) -> Result<HttpResponse> {
    let deck_id = req.match_info().get("id");
    match deck_id {
        Some(deck_id) => {
            let deck_id = Uuid::parse_str(deck_id);
            match deck_id {
                Ok(deck_id) => {
                    let mut connection = establish_connection();
                    let deck = Deck::find_by_id(deck_id, &mut connection);
                    match deck {
                        Some(deck) => {
                            deck.delete_from_db(&mut connection);
                            connection.close().ok();

                            Ok(HttpResponse::NoContent().finish())
                        }
                        None => Ok(HttpResponse::NotFound().json(JsonMessage { message: "Impossible de trouver ce paquet de carte." }))
                    }
                }
                Err(_) => Ok(HttpResponse::UnprocessableEntity().json(JsonMessage { message: "Paramètre incorrect." }))
            }
        }
        None => Ok(HttpResponse::MethodNotAllowed().json(JsonMessage { message: "Paramètre manquant." }))
    }
}