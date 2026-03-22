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
    pub content: String,
}

pub fn request_web() -> Connections {
    let date_str: chrono::format::DelayedFormat<chrono::format::StrftimeItems<'_>> =
        Local::now().format("%Y-%m-%d");
    let response: ApiResponse =
        reqwest::blocking::get(format!("https://www.nytimes.com/svc/connections/v2/{date_str}.json"))
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
                words: c.cards.iter().map(|c| c.content.clone()).collect(),
                hint: c.title.clone(),
                solved: false,
            }).collect(),
        solve_order: vec![],
        seed: 0,
    }
}
