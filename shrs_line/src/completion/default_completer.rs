use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use trie_rs::TrieBuilder;

use super::{all_files_completion, Completer, CompletionCtx};

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
