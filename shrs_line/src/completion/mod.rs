//! Shell autocompletion

mod better_completer;
pub use better_completer::*;

mod default_completer;
pub use default_completer::*;

mod utils;
pub use utils::*;

/// Context passed to completion handlers
pub struct CompletionCtx {
    /// The current argument we are on
    pub arg_num: usize,
}

pub trait Completer {
    fn complete(&self, buf: &str, ctx: CompletionCtx) -> Vec<String>;
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
