pub mod connections;
pub mod web;

use std::{
    cmp::{self, max},
    env,
    fs::File,
    io::{self, Read},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, Padding, Paragraph, Widget},
};

use crate::{connections::Connections, web::request_web};

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}

fn load_json(file_path: &str) -> Result<Connections, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let person: Connections = serde_json::from_str(&contents)?;
    Ok(person)
}

#[derive(Debug, Default)]
pub struct App {
    connections: Connections,
    selected: Vec<usize>,
    hovered: usize,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let args: Vec<String> = env::args().collect();

        self.connections = if args.len() == 2 {
            load_json(&args[1]).expect("Could not load level file.")
        } else {
            request_web()
        };

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if !key_event.is_press() {
            return;
        }
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('r') => {
                self.connections
                    .connections
                    .iter_mut()
                    .for_each(|c| c.solved = false);
                self.selected.clear();
                self.hovered = 0;
                self.connections.solve_order.clear();
            }
            KeyCode::Enter => {
                if self.selected.len() < 4 {
                    return;
                };
                let words = self.connections.get_words(self.connections.seed);
                self.connections.attempt_connection(
                    self.selected.iter().map(|&i| words[i].0.clone()).collect(),
                );
                self.selected.clear();
            }
            KeyCode::Char(' ') => {
                if self.selected.contains(&self.hovered) {
                    self.selected.retain(|i| i != &self.hovered);
                } else if self.selected.len() < 4 {
                    self.selected.push(self.hovered);
                }
            }
            KeyCode::Up => {
                if self.hovered >= 4 {
                    self.hovered -= 4
                }
            }
            KeyCode::Down => {
                if self.hovered < 12 {
                    self.hovered += 4
                }
            }
            KeyCode::Left => {
                if self.hovered % 4 > 0 {
                    self.hovered -= 1
                }
            }
            KeyCode::Right => {
                if self.hovered % 4 < 3 {
                    self.hovered += 1
                }
            }
            _ => {}
        }
        let num_solved: usize = self
            .connections
            .connections
            .iter()
            .fold(0, |acc, c| acc + c.solved as usize);
        if self.hovered / 4 < num_solved {
            self.hovered += 4;
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

const TILE_SMALL: (u16, u16) = (5, 11);
const TILE_LARGE: (u16, u16) = (7, 16);
const TILE_PADDING: u16 = 1;

enum Size {
    Small,
    Large,
}

impl Size {
    fn get_width(self: &Self) -> u16 {
        match self {
            Self::Small => TILE_SMALL.1,
            Self::Large => TILE_LARGE.1,
        }
    }

    fn get_height(self: &Self) -> u16 {
        match self {
            Self::Small => TILE_SMALL.0,
            Self::Large => TILE_LARGE.0,
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let words = self.connections.get_words(self.connections.seed);
        let longest_word_length = words.iter().fold(0, |l, (w, _, _)| cmp::max(l, w.len())) as u16;
        let longest_hint_length = self
            .connections
            .connections
            .iter()
            .map(|c| c.hint.clone())
            .fold(0, |l, h| cmp::max(l, h.len())) as u16;
        let size = if longest_word_length > TILE_SMALL.1 - TILE_PADDING * 2 {
            Size::Large
        } else {
            Size::Small
        };
        let mut col_constraints: Vec<Constraint> = (0..4)
            .map(|_| Constraint::Length(size.get_width()))
            .collect();
        col_constraints.push(Constraint::Length(longest_hint_length));
        let row_constraints = (0..4).map(|_| Constraint::Length(size.get_height()));
        let horizontal = Layout::horizontal(col_constraints)
            .spacing(2)
            .flex(Flex::Center);
        let vertical = Layout::vertical(row_constraints)
            .spacing(1)
            .flex(Flex::Center);
        let rows = vertical.split(area);
        let cells = rows
            .iter()
            .flat_map(|&row| horizontal.split(row)[0..4].to_vec());
        for (i, cell) in cells.enumerate() {
            let vertical_margin = cell.height.saturating_sub(1) / 2;
            let mut color = if words[i].2 {
                words[i].1.to_color()
            } else {
                0xFFFFFF
            };
            if self.hovered == i {
                color = color.saturating_sub(0x111111)
            }
            if self.selected.contains(&i) {
                color = color.saturating_sub(0x222222)
            }
            Paragraph::new(format!("{}", words[i].0))
                .block(
                    Block::new()
                        .padding(Padding::top(vertical_margin))
                        .bg(Color::from_u32(color)),
                )
                .fg(Color::Black)
                .centered()
                .render(cell, buf);
            if self.hovered == i {}
        }
        self.connections
            .solve_order
            .iter()
            .enumerate()
            .for_each(|(ri, ci)| {
                Paragraph::new(format!("{}", self.connections.connections[*ci].hint))
                    .block(Block::new().padding(Padding::new(
                        4,
                        0,
                        horizontal.split(rows[ri])[4].height.saturating_sub(1) / 2,
                        0,
                    )))
                    .render(horizontal.split(rows[ri])[4], buf)
            });
    }
}
