#[derive(Debug, PartialEq, Eq, Clone)]
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
    /// Select entire line (for Move action this behaves same as End)
    All,
    Find(char),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Action {
    Undo,
    Redo,
    Delete(Motion),
    Yank(Motion),
    Move(Motion),
    Insert,
    Chain(Box<Action>, Box<Action>),
    ToggleCase,
    Paste(Motion),
    LowerCase(Motion),
    UpperCase(Motion),
}
