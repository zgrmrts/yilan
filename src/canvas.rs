use super::element::Element;
use super::point::Point;
use crossterm::cursor;
use crossterm::style::{self, Colorize};
use crossterm::QueueableCommand;
use std::collections::hash_set::HashSet;
use std::io::{stdout, Stdout, Write};

pub struct Canvas {
    pub width: u16,
    pub height: u16,
    pub data: Vec<Element>,
    pub prev: Vec<Element>,
}

impl Canvas {
    pub fn new(width: u16, height: u16) -> Self {
        let mut data = Vec::new();
        let mut prev = Vec::new();
        (0..(width * height)).for_each(|_| {
            data.push(Element::Empty);
            prev.push(Element::Invalid);
        });
        Canvas {
            width,
            height,
            data,
            prev,
        }
    }
    pub fn get(&self, x: u16, y: u16) -> Element {
        return self.data[(x * self.height + y) as usize];
    }
    pub fn get_prev(&self, x: u16, y: u16) -> Element {
        return self.prev[(x * self.height + y) as usize];
    }
    pub fn set(&mut self, x: u16, y: u16, e: Element) {
        self.data[(x * self.height + y) as usize] = e;
    }
    pub fn clear(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set(x, y, Element::Empty);
            }
        }
    }
    pub fn save_to_prev(&mut self) {
        self.prev = self.data.clone();
    }
    pub fn diff_to_prev(&self) -> Vec<Point> {
        let mut v = Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                if self.get(x, y) != self.get_prev(x, y) {
                    v.push(Point::new(x, y));
                }
            }
        }
        v
    }
    fn draw_element(e: Element) -> style::PrintStyledContent<String> {
        match e {
            Element::SnakeHead => style::PrintStyledContent(String::from("█").yellow()),
            Element::SnakeBody => style::PrintStyledContent(String::from("█").blue()),
            Element::SnakeTail => style::PrintStyledContent(String::from("█").magenta()),
            Element::Apple => style::PrintStyledContent(String::from("█").red()),
            Element::Empty => style::PrintStyledContent(String::from("█").black()),
            Element::Character(c) => style::PrintStyledContent(c.to_string().white()),
            Element::Header => style::PrintStyledContent(String::from("█").grey()),
            Element::Invalid => panic!("Invalid character"),
        }
    }
    pub fn draw(&mut self, stdout: &mut Stdout) {
        let v = self.diff_to_prev();
        for Point { x, y } in v {
            let element = self.get(x, y);
            let styled_element = Canvas::draw_element(element);
            stdout
                .queue(cursor::MoveTo(x * 2, y))
                .expect("Can not move")
                .queue(&styled_element)
                .expect("Can not draw");
            if let Element::Character(_) = element {
                stdout
                    .queue(cursor::MoveTo(x * 2 + 1, y))
                    .expect("Can not move")
                    .queue(style::PrintStyledContent(String::from("_").black()))
                    .expect("Can not draw");
            } else {
                stdout
                    .queue(cursor::MoveTo(x * 2 + 1, y))
                    .expect("Can not move")
                    .queue(styled_element)
                    .expect("Can not draw");
            }
        }
        self.save_to_prev();
    }
}
