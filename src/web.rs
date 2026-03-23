use chrono::Local;
use serde::Deserialize;

use crate::connections::{Connection, ConnectionColor, Connections};

#[derive(Deserialize)]
pub struct ApiResponse {
    pub status: String,
    pub categories: Vec<Category>,
}

#[derive(Deserialize)]
pub struct Category {
    pub title: String,
    pub cards: Vec<Card>,
}

#[derive(Deserialize)]
pub struct Card {
    #[serde(default)]
    pub content: String,
    pub image_alt_text: String,
}

pub fn request_web() -> Connections {
    let date_str =
        Local::now().format("%Y-%m-%d");
    let response: ApiResponse = reqwest::blocking::get(format!(
        "https://www.nytimes.com/svc/connections/v2/{date_str}.json"
    ))
    .expect("Could not fetch content from NY Times.")
    .json()
    .expect("Could not parse fetched content from NY Times.");
    Connections {
        connections: response
            .categories
            .iter()
            .enumerate()
            .map(|(i, c)| Connection {
                color: ConnectionColor::from_id(i),
                words: c
                    .cards
                    .iter()
                    .map(|c| {
                        if c.content.len() > 0 {
                            c.content.clone()
                        } else {
                            c.image_alt_text.clone()
                        }
                    })
                    .collect(),
                hint: c.title.clone(),
                solved: false,
            })
            .collect(),
        solve_order: vec![],
        seed: 0,
    }
}
