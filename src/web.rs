use chrono::{DateTime, Local};
use serde::Deserialize;

use crate::connections::{Connection, ConnectionColor, Connections};

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub categories: Vec<Category>,
}

#[derive(Deserialize, Debug)]
pub struct Category {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub cards: Vec<Card>,
}

#[derive(Deserialize, Debug)]
pub struct Card {
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub image_alt_text: String,
}

pub fn request_web() -> Connections {
    let date = Local::now();
    request_web_date(date)
}

pub fn request_web_date(date: DateTime<Local>) -> Connections {
    let date_str = date.format("%Y-%m-%d");
    let response: ApiResponse = reqwest::blocking::get(format!(
        "https://www.nytimes.com/svc/connections/v2/{date_str}.json"
    ))
    .expect("Could not fetch content from NY Times.")
    .json()
    .expect("Could not parse fetched content from NY Times.");
    if response.status != "OK" {
        panic!("API error for date string {}.", date_str);
    }
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
        title: date_str.to_string(),
        solve_order: vec![],
        seed: 0,
    }
}
