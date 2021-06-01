use super::canvas::Canvas;
use super::direction::Direction;
use super::element::Element;
use super::keymon::key_monitor;
use super::point::add;
use super::point::Point;
use crossterm::event::KeyCode;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, terminal, QueueableCommand};
use std::collections::VecDeque;
use std::convert::TryInto;
use std::io::{stdout, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Game {
    snake: VecDeque<Point>,
    direction: Direction,
    speed: u16,
    apple: Point,
    game_canvas: Canvas,
    header_canvas: Canvas,
    stdout: std::io::Stdout,
    game_width: u16,
    game_height: u16,
    score: u16,
    elongate: u16,
}
const HEADER_HEIGHT: u16 = 1;
const ELONGATE_CONST: u16 = 2;

impl Game {
    pub fn new() -> Self {
        let (width, height) = terminal::size().unwrap();
        let game_height = height - HEADER_HEIGHT;
        let game_width = width / 2;
        let game_canvas = Canvas::new(0, HEADER_HEIGHT, game_width, game_height, 2);
        let header_canvas = Canvas::new(0, 0, width, HEADER_HEIGHT, 1);
        let stdout = stdout();
        let mut snake = VecDeque::new();
        let delta = Direction::Down.delta();
        let mut snake_segment = Game::random_point_init(game_width, game_height);
        for _ in 0..6 {
            snake.push_back(snake_segment);
            snake_segment = add(&snake_segment, &delta, game_width, game_height);
        }
        Game {
            snake,
            direction: Direction::Up,
            speed: 5u16,
            apple: Game::random_point_init(game_width, game_height),
            game_canvas,
            header_canvas,
            stdout,
            game_width,
            game_height,
            score: 0,
            elongate: 0,
        }
    }
    fn clear(&mut self) {
        self.game_canvas.clear();
        self.game_canvas.draw(&mut self.stdout);
        self.header_canvas.clear();
        self.header_canvas.draw(&mut self.stdout);
        self.stdout.flush().unwrap();
    }
    fn draw(&mut self) {
        // game
        self.game_canvas.clear();
        for Point { x, y } in self.snake.iter() {
            self.game_canvas.set(*x, *y, Element::SnakeBody);
        }
        self.game_canvas
            .set(self.apple.x, self.apple.y, Element::Apple);
        self.game_canvas
            .set(self.head().x, self.head().y, Element::SnakeHead);
        self.game_canvas
            .set(self.tail().x, self.tail().y, Element::SnakeTail);
        let _count = self.game_canvas.draw(&mut self.stdout);
        // header
        self.header_canvas.clear();
        let score_string = format!("Score: {}     press q to quit", self.score);
        for (i, c) in score_string.chars().enumerate() {
            self.header_canvas
                .set(i.try_into().unwrap(), 0, Element::Character(c));
        }
        self.header_canvas.draw(&mut self.stdout);
        // flush
        self.stdout.flush().unwrap();
    }
    fn head(&self) -> &Point {
        self.snake.front().unwrap()
    }
    fn tail(&self) -> &Point {
        self.snake.back().unwrap()
    }
    fn random_point_init(width: u16, height: u16) -> Point {
        Point::new(
            rand::random::<u16>() % width,
            rand::random::<u16>() % height,
        )
    }
    fn random_point(&self) -> Point {
        Game::random_point_init(self.game_width, self.game_height)
    }
    fn tick(&mut self) {
        let delta = self.direction.delta();
        let new_head = add(&self.head(), &delta, self.game_width, self.game_height);
        if self.snake.contains(&new_head) {
            self.game_over(false);
        }
        self.snake.push_front(new_head);
        if new_head == self.apple {
            self.elongate += ELONGATE_CONST;
            loop {
                self.apple = self.random_point();
                if !self.snake.contains(&self.apple) {
                    break;
                }
            }
            self.speed += 1;
            self.score += 1;
        }
        if self.elongate > 0 {
            self.elongate -= 1;
        } else {
            self.snake.pop_back();
        }
    }
    fn game_over(&mut self, quit: bool) {
        if !quit {
            thread::sleep(Duration::from_millis(1000));
        }
        disable_raw_mode().unwrap();
        self.stdout
            .queue(cursor::MoveTo(0, 0))
            .unwrap()
            .queue(cursor::Show)
            .unwrap()
            .queue(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        if !quit {
            println!("Game over. Score : {}", self.score);
        }
        std::process::exit(0);
    }
    pub fn main_loop(&mut self) {
        let event_queue_mutex: Arc<Mutex<VecDeque<KeyCode>>> =
            Arc::new(Mutex::new(VecDeque::<KeyCode>::new()));

        let event_queue_mutex_clone = Arc::clone(&event_queue_mutex);
        thread::spawn(move || {
            key_monitor(event_queue_mutex_clone);
        });
        self.stdout.queue(cursor::Hide).unwrap();
        enable_raw_mode().unwrap();
        self.clear();
        loop {
            self.draw();
            thread::sleep(Duration::from_millis((400 / self.speed).into()));
            let mut q = event_queue_mutex.lock().unwrap();
            loop {
                if let Some(k) = q.pop_front() {
                    if let Some(d) = Game::keycode_to_direction(&k) {
                        if d == self.direction.inverse() {
                            continue;
                        }
                        self.direction = d;
                        break;
                    } else if let KeyCode::Char('q') = k {
                        self.game_over(true);
                    }
                }
                break;
            }
            self.tick();
        }
    }
    fn keycode_to_direction(k: &KeyCode) -> Option<Direction> {
        match k {
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            KeyCode::Up => Some(Direction::Up),
            _ => None,
        }
    }
}
