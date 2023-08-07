//! Structs that make up the parsed AST of the POSIX shell language

/// File redirection
#[derive(Debug, Clone)]
pub struct Redirect {
    pub n: Option<usize>,
    pub file: String,
    pub mode: RedirectMode,
}

/// File redirection modes
#[derive(Debug, Clone)]
pub enum RedirectMode {
    Read,
    Write,
    ReadAppend,
    WriteAppend,
    ReadDup,
    WriteDup,
    ReadWrite,
}

/// Assignment
#[derive(Debug, Clone)]
pub struct Assign {
    pub var: String,
    pub val: String,
}

/// Separator character between commands
#[derive(Debug, Clone)]
pub enum SeparatorOp {
    /// Ampersand (&)
    Amp,
    /// Semicolon (;)
    Semi,
}

#[derive(Debug, Clone)]
pub enum Command {
    /// Basic command
    ///
    /// ```sh
    /// ls -al
    /// ```
    Simple {
        assigns: Vec<Assign>,
        redirects: Vec<Redirect>,
        args: Vec<String>,
    },

    /// Two commands joined by a pipe
    ///
    /// ```sh
    /// cat .bashrc | wc -l
    /// ```
    Pipeline(Box<Command>, Box<Command>),

    /// Compound command of And
    And(Box<Command>, Box<Command>),

    /// Compound command of Or
    Or(Box<Command>, Box<Command>),

    /// Negate the exit code of command
    Not(Box<Command>),

    /// Asynchronous list of commands
    ///
    /// ```sh
    /// command1 & command2
    /// ```
    /// We do not wait for `command1` to finish executing before executing `command2`
    AsyncList(Box<Command>, Option<Box<Command>>),

    /// Sequential list of commands
    /// ```sh
    /// command1 ; command2
    /// ```
    /// We wait for `command1` to finish executing before executing `command2`
    SeqList(Box<Command>, Option<Box<Command>>),

    /// Subshell for command to run
    /// ```sh
    /// (cd src && ls)
    /// ```
    Subshell(Box<Command>),

    /// If statements
    If {
        conds: Vec<Condition>,
        else_part: Option<Box<Command>>,
    },

    /// While statements
    While {
        cond: Box<Command>,
        body: Box<Command>,
    },

    /// Until statements
    Until {
        cond: Box<Command>,
        body: Box<Command>,
    },

    /// For loops
    For {
        name: String,
        wordlist: Vec<String>,
        body: Box<Command>,
    },

    /// Case statements
    Case { word: String, arms: Vec<CaseArm> },

    /// Function definition
    Fn { fname: String, body: Box<Command> },

    /// No op
    None,
}

/// Represents each match arm in case statement
#[derive(Debug, Clone)]
pub struct CaseArm {
    pub pattern: Vec<String>,
    pub body: Box<Command>,
}

/// Corresponds to a condition followed by a body to execute in an 'if' or 'elif' block
#[derive(Debug, Clone)]
pub struct Condition {
    pub cond: Box<Command>,
    pub body: Box<Command>,
}
