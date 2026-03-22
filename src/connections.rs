use color_eyre::owo_colors::colors::Yellow;
use rand::{SeedableRng, rngs::StdRng, seq::SliceRandom};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum ConnectionColor {
    #[default]
    Yellow,
    Green,
    Blue,
    Purple,
}

impl ConnectionColor {
    pub fn to_color(self: &Self) -> u32 {
        match self {
            Self::Yellow => 0xF9DF6E,
            Self::Green => 0xA0C459,
            Self::Blue => 0xB1C4EF,
            Self::Purple => 0xBA81C5,
        }
    }

    pub fn from_id(id: usize) -> ConnectionColor {
        match id {
            0 => ConnectionColor::Yellow,
            1 => ConnectionColor::Green,
            2 => ConnectionColor::Blue,
            _ => ConnectionColor::Purple,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub color: ConnectionColor,
    pub words: Vec<String>,
    pub hint: String,
    #[serde(default)]
    pub solved: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Connections {
    pub connections: Vec<Connection>,
    #[serde(default)]
    pub solve_order: Vec<usize>,
    pub seed: u64,
}

impl Connections {
    pub fn get_words(self: &Self, seed: u64) -> Vec<(String, ConnectionColor, bool)> {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut words: Vec<(String, ConnectionColor, bool)> = self
            .connections
            .iter()
            .filter(|c| !c.solved)
            .map(|c| c.words.iter().map(|w| (w.clone(), c.color.clone())))
            .flatten()
            .map(|(w, c)| (w, c, false))
            .collect();
        words.shuffle(&mut rng);
        let solved_words: Vec<(String, ConnectionColor, bool)> = self
            .solve_order
            .iter()
            .map(|i| {
                self.connections[*i]
                    .words
                    .iter()
                    .map(|w| (w.clone(), self.connections[*i].color.clone()))
            })
            .flatten()
            .map(|(w, c)| (w, c, true))
            .collect();
        words.splice(0..0, solved_words);
        words
    }

    pub fn attempt_connection(self: &mut Self, words: Vec<String>) -> bool {
        let mut success = false;
        for (i, conn) in &mut self.connections.iter_mut().enumerate() {
            let mut matches = 0;
            for w in &words {
                if conn.words.contains(w) {
                    matches += 1;
                }
            }
            if matches == 4 {
                conn.solved = true;
                success = true;
                self.solve_order.push(i);
                break;
            }
        }
        success
    }
}
