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
            .take(10) // TODO make this config option
            .collect();

        results
    }
}

#[cfg(test)]
mod tests {
    use super::{Completer, DefaultCompleter};

    /*
    #[test]
    fn default_completer() {
        let completer = DefaultCompleter::new(vec![
            "ls", "ld", "ldd", "let", "ln", "lsblk", "lscpu", "lspci", "lsusb",
        ]);
        completer.complete("ls", 0);
    }
    */
}
