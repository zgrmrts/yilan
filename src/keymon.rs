use crossterm::event::read;
use crossterm::event::Event::Key;
use crossterm::event::KeyCode;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub fn key_monitor(event_queue: Arc<Mutex<VecDeque<KeyCode>>>) {
    loop {
        match read().unwrap() {
            Key(k) => match k.code {
                KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::Up
                | KeyCode::Char('q') => {
                    event_queue.lock().unwrap().push_back(k.code);
                }
                _ => (),
            },
            _ => (),
        }
    }
}
