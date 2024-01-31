//! Shell autocompletion

mod completer;
pub use completer::*;

mod utils;
// TODO: Report bugged warning (this reexport is required)
#[allow(unused_imports)]
pub use utils::*;

mod data;

/// How should the completion be substituted
#[derive(Clone)]
pub enum ReplaceMethod {
    /// Append the returned value after the cursor
    Append,
    /// Replace the last word
    Replace,
}

#[derive(Clone)]
pub struct Completion {
    /// If space should be added after completion
    pub add_space: bool,
    /// Vanity value that can be used by menu or others to display friendly version of completion
    pub display: Option<String>,
    /// Actual value to perform completion with
    pub completion: String,
    /// Replace method
    pub replace_method: ReplaceMethod,
    /// Additional helpful information about the completion
    pub comment: Option<String>,
}

impl Completion {
    /// Used to provide user friendly preview of what value will be completed
    pub fn display(&self) -> String {
        self.display.clone().unwrap_or(self.completion.clone())
    }
    /// Get actual value to be used when accepting completion
    pub fn accept(&self) -> String {
        let mut output = self.completion.clone();
        if self.add_space {
            output += " ";
        }
        output
    }
}

/// Implement this trait to define your own tab completion system
pub trait Completer {
    /// Given context on the current state of the input, output list of possible completions
    fn complete(&self, ctx: &CompletionCtx) -> Vec<Completion>;
}

pub struct CompletionCtx {
    /// The currently entered line split by arguments
    ///
    /// The cursor position is after the very last argument
    line: Vec<String>,
}

impl CompletionCtx {
    pub fn new(line: Vec<String>) -> Self {
        Self { line }
    }

    /// Get the name of the command
    pub fn cmd_name(&self) -> Option<&String> {
        self.line.get(0)
    }

    /// Get the word that the user is currently typing
    pub fn cur_word(&self) -> Option<&String> {
        self.line.last()
    }

    /// Which argument are we currently on
    pub fn arg_num(&self) -> usize {
        self.line.len().saturating_sub(1)
    }
}

#[cfg(test)]
mod tests {

    /*
    #[test]
    fn default_completer() {
        let completer = DefaultCompleter::new(vec![
            "ls", "ld", "ldd", "let", "ln", "lsblk", "lscpu", "lspci", "lsusb",
        ]);
        completer.complete("ls", 0);
    }
    */

    /*
    use super::filepath_completion_p;

    #[test]
    fn test_filepath_completion() {
        let out = filepath_completion_p("/home/pinosaur", |f| f.file_type().unwrap().is_file());
        println!("{:?}", out);
    }
    */
}
