use super::element::Element;
use super::point::Point;
use crossterm::cursor;
use crossterm::style::{self, Colorize};
use crossterm::QueueableCommand;
use std::io::Stdout;

pub struct Canvas {
    offset_x: u16,
    offset_y: u16,
    pub width: u16,
    pub height: u16,
    horizontal_mult: u16,
    data: Vec<Element>,
    prev_data: Vec<Element>,
}

impl Canvas {
    pub fn new(
        offset_x: u16,
        offset_y: u16,
        width: u16,
        height: u16,
        horizontal_mult: u16,
    ) -> Self {
        let mut data = Vec::new();
        let mut prev_data = Vec::new();
        (0..width).for_each(|_| {
            (0..height).for_each(|_| {
                data.push(Element::Empty);
                prev_data.push(Element::Invalid);
            });
        });
        Canvas {
            offset_x,
            offset_y,
            width,
            height,
            horizontal_mult,
            data,
            prev_data,
        }
    }
    pub fn get(&self, x: u16, y: u16) -> Element {
        return self.data[(x * self.height + y) as usize];
    }
    fn get_prev(&self, x: u16, y: u16) -> Element {
        return self.prev_data[(x * self.height + y) as usize];
    }
    pub fn set(&mut self, x: u16, y: u16, e: Element) {
        if self.data[(x * self.height + y) as usize] != e {
            self.data[(x * self.height + y) as usize] = e;
        }
    }
    pub fn clear(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set(x, y, Element::Empty);
            }
        }
    }
    fn save_to_prev(&mut self) {
        self.prev_data = self.data.clone();
    }
    fn diff_to_prev(&self) -> Vec<Point> {
        let mut v = Vec::new();
        (0..self.width).for_each(|x| {
            (0..self.height).for_each(|y| {
                if self.get(x, y) != self.get_prev(x, y) {
                    v.push(Point::new(x, y));
                }
            })
        });
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
            Element::Invalid => panic!("Invalid element"),
        }
    }
    pub fn draw(&mut self, stdout: &mut Stdout) -> i32 {
        let mut count = 0;
        for Point { x, y } in self.diff_to_prev().iter() {
            count = count + 1;
            let element = self.get(*x, *y);
            let styled_element = Canvas::draw_element(element);
            for h in 0..self.horizontal_mult {
                stdout
                    .queue(cursor::MoveTo(
                        self.offset_x + x * self.horizontal_mult + h,
                        self.offset_y + *y,
                    ))
                    .expect("Can not move")
                    .queue(&styled_element)
                    .expect("Can not draw");
            }
        }
        self.save_to_prev();
        count
    }
}
