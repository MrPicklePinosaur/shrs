#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Command {
    pub repeat: u32,
    pub action: Action,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Motion {
    None,
    BackWord,
    WordPunc,
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
    Find(char),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Action {
    Delete(Motion),
    Change(Motion),
    Yank(Motion),
    Move(Motion),
    Insert(Motion),
}
