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
    let image= format!("{}/cards/{}.svg", get_asset_url(), motif + 1);

    CardRepresentation { name, image }
}

fn get_asset_url() -> String {
    return env::var("ASSET_URL")
        .expect("ASSET_URL must be set");
}