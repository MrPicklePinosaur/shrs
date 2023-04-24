//! Shell autocompletion

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use trie_rs::TrieBuilder;

mod engine;

// also provide some commonly used completion lists
// - directories
// - executables
// - file extension
// - filename regex
// - known hosts

/// Context passed to completion handlers
pub struct CompletionCtx {
    /// The current argument we are on
    pub arg_num: usize,
}

pub trait Completer {
    fn complete(&self, buf: &str, ctx: CompletionCtx) -> Vec<String>;
}

/// Very basic completer that uses prefix tree to match on a predefined word list
pub struct DefaultCompleter {
    wordlist: Vec<String>,
}

// TODO next step, make word list vary with context
// TODO differ between cmdname, args etc
impl DefaultCompleter {
    pub fn new(wordlist: Vec<String>) -> Self {
        DefaultCompleter { wordlist }
    }
}

impl Completer for DefaultCompleter {
    fn complete(&self, buf: &str, ctx: CompletionCtx) -> Vec<String> {
        if ctx.arg_num == 1 {
            // Return all results if empty query
            if buf.is_empty() {
                return self.wordlist.clone();
            }

            // TODO waste to keep building wordlist
            let mut builder = TrieBuilder::new();
            for word in &self.wordlist {
                builder.push(word);
            }
            let trie = builder.build();

            // complete command name from path if is first argument
            let results = trie.predictive_search(buf);
            let results: Vec<String> = results
                .iter()
                .map(|x| std::str::from_utf8(x).unwrap().to_string())
                .collect();

            results
        } else {
            // Return all results if empty query
            if buf.is_empty() {
                return all_files_completion(&std::env::current_dir().unwrap()).unwrap();
            }

            let buf_path = PathBuf::from(buf);

            // convert to absolute
            let dir = if buf_path.is_absolute() {
                buf_path.clone()
            } else {
                // TODO not sure if should rely on env working dir
                let pwd = std::env::current_dir().unwrap();
                pwd.join(buf_path.clone())
            };

            let suffix = dir.file_name().unwrap();
            let prefix = dir.parent().unwrap_or(&dir);

            let files = all_files_completion(prefix).unwrap();

            // TODO is this too expensive?
            let mut builder = TrieBuilder::new();
            for file in files {
                builder.push(file);
            }
            let trie = builder.build();

            // TODO this is dumb
            let mut display_prefix = buf_path.parent().unwrap().display().to_string();
            // append backslash to end if non empty
            if !display_prefix.is_empty() {
                display_prefix.push('/');
            }

            let results = trie.predictive_search(suffix.to_str().unwrap());
            let results: Vec<String> = results
                .iter()
                .map(|x| std::str::from_utf8(x).unwrap().to_string())
                .map(|x| format!("{display_prefix}{x}"))
                .map(|x|
                     // append trailing slash if path is directory
                     if PathBuf::from(x.clone()).is_dir() {
                         format!("{x}/")
                     } else {
                         x
                     }
                 )
                .collect();

            results
        }
    }
}

/// Generate list of files in the current working directory with predicate
pub fn filepath_completion_p<P>(dir: &Path, predicate: P) -> std::io::Result<Vec<String>>
where
    P: FnMut(&std::fs::DirEntry) -> bool,
{
    use std::fs;

    let out: Vec<String> = fs::read_dir(dir)?
        .filter_map(|f| f.ok())
        .filter(predicate)
        .map(|f| f.file_name().into_string())
        .filter_map(|f| f.ok())
        .collect();

    Ok(out)
}

/// Generate list of files in the current working directory
pub fn all_files_completion(dir: &Path) -> std::io::Result<Vec<String>> {
    filepath_completion_p(dir, |_| true)
}

/// Generate list of all executables in PATH
pub fn exectuable_completion(_dir: &Path) -> std::io::Result<Vec<String>> {
    todo!()
}

/// Generate list of all ssh hosts
pub fn ssh_completion(_dir: &Path) -> std::io::Result<Vec<String>> {
    todo!()
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
