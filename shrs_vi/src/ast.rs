#[derive(Debug)]
pub struct Command {
    pub repeat: u32,
    pub action: Action,
}

#[derive(Debug)]
pub enum Motion {
    Word,
    Left,
    Right,
}

#[derive(Debug)]
pub enum Action {
    Delete(Motion),
    Change(Motion),
    Yank(Motion),
    Move(Motion),
    MoveLeft,
    MoveRight,
    MoveStart,
    MoveEnd,
    /// Move cursor to point to the next character found
    MoveFindChar(char),
    MoveNextWord,
    MoveBackWord,
}
