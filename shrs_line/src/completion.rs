use trie_rs::{Trie, TrieBuilder};

pub struct Completion {}

// also provide some commonly used completion lists
// - directories
// - executables
// - file extension
// - filename regex
// - known hosts

pub trait Completer {
    fn complete(&self, buf: &str) -> Vec<String>;
}

pub struct DefaultCompleter {
    completions: Trie<u8>,
}

/// Very basic completer that uses prefix tree to match on a predefined word list
// TODO next step, make word list vary with context
// TODO differ between cmdname, args etc
impl DefaultCompleter {
    pub fn new(wordlist: Vec<String>) -> Self {
        // build prefix tree from wordlist
        let mut builder = TrieBuilder::new();
        for word in wordlist {
            builder.push(word);
        }
        let trie = builder.build();

        DefaultCompleter { completions: trie }
    }
}

impl Completer for DefaultCompleter {
    fn complete(&self, buf: &str) -> Vec<String> {
        if buf.is_empty() {
            return vec![];
        }
        let results = self.completions.predictive_search(buf);
        let results: Vec<String> = results
            .iter()
            .map(|x| std::str::from_utf8(x).unwrap().to_string())
            .collect();

        results
    }
}

pub fn filepath_completion_p<P>(dir: &str, predicate: P) -> std::io::Result<Vec<String>>
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

pub fn all_files_completion(dir: &str) -> std::io::Result<Vec<String>> {
    filepath_completion_p(dir, |_| true)
}

pub fn exectuable_completion(dir: &str) -> std::io::Result<Vec<String>> {
    todo!()
}

pub fn ssh_completion(dir: &str) -> std::io::Result<Vec<String>> {
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

    use super::filepath_completion_p;

    #[test]
    fn test_filepath_completion() {
        let out = filepath_completion_p("/home/pinosaur", |f| f.file_type().unwrap().is_file());
        println!("{:?}", out);
    }
}
