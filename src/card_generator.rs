use std::env;
use crate::{Card, CardRepresentation};

pub fn get_card(motif: &Card) -> CardRepresentation {
    let card_color = match motif / 13 {
        0 => "Trèfle",
        1 => "Carreau",
        2 => "Coeur",
        3 => "Pique",
        _ => panic!("Incorrect value.")
    };

    let card_value = match motif % 13 {
        0 => "As",
        1 => "2",
        2 => "3",
        3 => "4",
        4 => "5",
        5 => "6",
        6 => "7",
        7 => "8",
        8 => "9",
        9 => "10",
        10 => "Valet",
        11 => "Reine",
        12 => "Roi",
        _ => panic!("Incorrect value.")
    };

    let name = format!("{} de {}", card_value, card_color);
    let image= format!("{}{}{}", get_asset_url(), motif, get_asset_extension());

    CardRepresentation { name, image, value: *motif }
}

fn get_asset_url() -> String {
    return env::var("ASSET_URL")
        .expect("ASSET_URL must be set");
}

fn get_asset_extension() -> String {
    return env::var("ASSET_EXTENSION")
        .expect("ASSET_EXTENSION must be set");
}