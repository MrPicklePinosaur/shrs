//! Shell autocompletion

mod completer;
pub use completer::*;

mod utils;
pub use utils::*;

mod context;
pub use context::*;

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
    pub(crate) add_space: bool,
    /// Vanity value that can be used by menu or others to display friendly version of completion
    pub(crate) display: Option<String>,
    /// Actual value to perform completion with
    pub(crate) completion: String,
    /// Replace method
    pub(crate) replace_method: ReplaceMethod,
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

pub trait Completer {
    fn complete(&self, ctx: &CompletionCtx) -> Vec<Completion>;
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
