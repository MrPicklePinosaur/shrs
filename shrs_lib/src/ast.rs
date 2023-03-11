#[derive(Debug)]
pub struct Redirect<'input> {
    pub n: Option<usize>,
    pub file: &'input str,
    pub mode: RedirectMode,
}

#[derive(Debug)]
pub enum RedirectMode {
    Read,
    Write,
    ReadAppend,
    WriteAppend,
    ReadDup,
    WriteDup,
    ReadWrite,
}

#[derive(Debug)]
pub struct Assign<'input> {
    pub var: &'input str,
    pub val: &'input str,
}

/// Seperator character between commands
#[derive(Debug)]
pub enum SeperatorOp {
    /// Ampersand (&)
    Amp,
    /// Semicolon (;)
    Semi,
}

#[derive(Debug)]
pub enum Command<'input> {
    /// Basic command
    ///
    /// ```sh
    /// ls -al
    /// ```
    Simple {
        assigns: Vec<Assign<'input>>,
        redirects: Vec<Redirect<'input>>,
        args: Vec<String>,
    },

    /// Two commands joined by a pipe
    ///
    /// ```sh
    /// cat .bashrc | wc -l
    /// ```
    Pipeline(Box<Command<'input>>, Box<Command<'input>>),

    /// Compound command of And
    And(Box<Command<'input>>, Box<Command<'input>>),

    /// Compound command of Or
    Or(Box<Command<'input>>, Box<Command<'input>>),

    /// Negate the exit code of command
    Not(Box<Command<'input>>),

    /// Asynchronous list of commands
    ///
    /// ```sh
    /// command1 & command2
    /// ```
    /// We do not wait for `command1` to finish executing before executing `command2`
    AsyncList(Box<Command<'input>>, Option<Box<Command<'input>>>),

    /// Sequential list of commands
    /// ```sh
    /// command1 ; command2
    /// ```
    /// We wait for `command1` to finish executing before executing `command2`
    SeqList(Box<Command<'input>>, Option<Box<Command<'input>>>),

    /// Subshell for command to run
    /// ```sh
    /// (cd src && ls)
    /// ```
    Subshell(Box<Command<'input>>),

    /// If statements
    If {
        conds: Vec<Condition<'input>>,
        else_part: Option<Box<Command<'input>>>,
    },

    /// While statements
    While {
        cond: Box<Command<'input>>,
        body: Box<Command<'input>>,
    },

    /// Until statements
    Until {
        cond: Box<Command<'input>>,
        body: Box<Command<'input>>,
    },

    /// For loops
    For {
        name: &'input str,
        wordlist: Vec<&'input str>,
        body: Box<Command<'input>>,
    },

    /// Case statements
    Case {
        word: &'input str,
        arms: Vec<CaseArm<'input>>,
    },

    Fn {
        fname: &'input str,
        body: Box<Command<'input>>,
    },

    /// No op
    None,
}

#[derive(Debug)]
pub struct CaseArm<'input> {
    pub pattern: Vec<&'input str>,
    pub body: Box<Command<'input>>,
}

/// Corresponds to a condition followed by a body to execute in an 'if' or 'elif' block
#[derive(Debug)]
pub struct Condition<'input> {
    pub cond: Box<Command<'input>>,
    pub body: Box<Command<'input>>,
}
