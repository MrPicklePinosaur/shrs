#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Command {
    pub repeat: u32,
    pub action: Action,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Motion {
    BackWord,
    Word,
    Left,
    Right,
    Start,
    Up,
    Down,
    End,
    /// The current character
    Char,
    /// Select entire line (for Move action this behaves same as End)
    All,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Action {
    Delete(Motion),
    Change(Motion),
    Yank(Motion),
    Move(Motion),
    Insert,
}
