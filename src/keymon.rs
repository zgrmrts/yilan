use crossterm::event::read;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub fn key_monitor(event_queue: Arc<Mutex<VecDeque<crossterm::event::KeyCode>>>) {
    loop {
        match read().unwrap() {
            crossterm::event::Event::Key(k) => match k.code {
                crossterm::event::KeyCode::Down
                | crossterm::event::KeyCode::Left
                | crossterm::event::KeyCode::Right
                | crossterm::event::KeyCode::Up
                | crossterm::event::KeyCode::Char('q') => {
                    event_queue.lock().unwrap().push_back(k.code);
                }
                _ => (),
            },
            _ => (),
        }
    }
}
