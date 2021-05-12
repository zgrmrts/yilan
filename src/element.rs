#[derive(Clone, Copy, PartialEq)]
pub enum Element {
    SnakeHead,
    SnakeBody,
    SnakeTail,
    Apple,
    Empty,
    Character(char),
    Header,
    Invalid,
}
