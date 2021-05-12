use super::canvas::Canvas;
use super::direction::Direction;
use super::element::Element;
use super::point::add;
use super::point::Point;
use std::collections::VecDeque;
use std::convert::TryInto;

extern crate crossterm;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{
    cursor,
    style::{self, Color, Colorize, Print, SetBackgroundColor, SetForegroundColor},
    terminal, QueueableCommand, Result,
};
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

pub struct Game {
    snake: VecDeque<Point>,
    direction: Direction,
    speed: u16,
    apple: Point,
    can: Canvas,
    stdout: std::io::Stdout,
    game_width: u16,
    game_height: u16,
    score: u16,
}
impl Game {
    pub fn new() -> Self {
        let (width, height) = terminal::size().unwrap();
        let game_height = height - 1;
        let game_width = width / 2;
        let can = Canvas::new(game_width, height);
        drop(width);
        drop(height);
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
            can,
            stdout,
            game_width,
            game_height,
            score: 0,
        }
    }
    fn clear(&mut self) {
        self.can.clear();
        self.can.draw(&mut self.stdout);
        self.stdout.flush().expect("stdout flush error");
    }
    fn draw(&mut self) {
        self.can.clear();
        for i in 0..self.game_width {
            self.can.set(i, 0, Element::Header);
        }
        let score_string = format!("Score: {}   press q to quit", self.score);
        for (i, c) in score_string.chars().enumerate() {
            self.can
                .set(i.try_into().expect("x"), 0, Element::Character(c));
        }
        for Point { x, y } in self.snake.iter() {
            self.can.set(*x, *y + 1, Element::SnakeBody);
        }
        self.can.set(self.apple.x, self.apple.y + 1, Element::Apple);
        self.can
            .set(self.head().x, self.head().y + 1, Element::SnakeHead);
        self.can
            .set(self.tail().x, self.tail().y + 1, Element::SnakeTail);
        self.can.draw(&mut self.stdout);
        self.stdout.flush().expect("stdout flush error");
    }
    fn head(&self) -> Point {
        self.snake[0]
    }
    fn tail(&self) -> Point {
        self.snake[self.snake.len() - 1]
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
        let mut apple_eaten = false;
        let new_head = add(&self.head(), &delta, self.game_width, self.game_height);
        if self.snake.contains(&new_head) {
            self.game_over(false);
        }
        self.snake.push_front(new_head);
        if new_head == self.apple {
            apple_eaten = true;
            self.score = self.score + 1;
        }
        if apple_eaten {
            self.apple = self.random_point();
            self.speed = self.speed + 1;
        } else {
            self.snake.pop_back();
        }
    }
    fn game_over(&mut self, quit: bool) {
        if !quit {
            thread::sleep(Duration::from_millis(1000));
        }
        crossterm::terminal::disable_raw_mode().expect("Can not disable raw mode");
        self.stdout
            .queue(cursor::MoveTo(0, 0))
            .expect("Can not move")
            .queue(cursor::Show)
            .expect("Can not show the cursor")
            .queue(terminal::Clear(terminal::ClearType::All))
            .expect("Can not clear");
        if !quit {
            println!("Game over. Score : {}", self.score);
        }
        std::process::exit(0);
    }
    pub fn main_loop(&mut self) {
        self.stdout
            .queue(cursor::Hide)
            .expect("Can not hide cursor");
        crossterm::terminal::enable_raw_mode().expect("Can not enable raw mode");
        self.clear();
        loop {
            self.draw();
            // thread::sleep(Duration::from_millis());
            if crossterm::event::poll(Duration::from_millis((400 / self.speed).into()))
                .expect("Can not poll")
            {
                match read().expect("Can not read console input") {
                    crossterm::event::Event::Key(k) => match k.code {
                        crossterm::event::KeyCode::Down => {
                            if self.direction != Direction::Up {
                                self.direction = Direction::Down
                            }
                        }
                        crossterm::event::KeyCode::Left => {
                            if self.direction != Direction::Right {
                                self.direction = Direction::Left
                            }
                        }
                        crossterm::event::KeyCode::Right => {
                            if self.direction != Direction::Left {
                                self.direction = Direction::Right
                            }
                        }
                        crossterm::event::KeyCode::Up => {
                            if self.direction != Direction::Down {
                                self.direction = Direction::Up
                            }
                        }
                        crossterm::event::KeyCode::Char('q') => {
                            self.game_over(true);
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
            self.tick();
        }
    }
}
