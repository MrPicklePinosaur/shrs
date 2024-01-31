//! Misc algorithms

// TODO maybe this solution could use some optimization
// Should be O(nlogn + m), where m is length of longest string in input
pub fn longest_common_prefix(input: Vec<&str>) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut input = input;
    input.sort();

    let mut prefix = String::new();

    let first = input.first().unwrap();
    let last = input.last().unwrap();
    let min_len = std::cmp::min(first.len(), last.len());
    for i in 0..min_len {
        let first_i = first.chars().nth(i).unwrap();
        let last_i = last.chars().nth(i).unwrap();

        if first_i != last_i {
            break;
        }

        prefix += &first_i.to_string();
    }

    prefix
}

#[cfg(test)]
mod tests {
    use super::longest_common_prefix;

    #[test]
    fn test_longest_common_prefix() {
        let prefix = longest_common_prefix(vec!["aaa", "aab", "aac"]);
        assert_eq!(prefix, "aa");

        let prefix = longest_common_prefix(vec!["aaa", "bbb", "ccc"]);
        assert_eq!(prefix, "");
    }
}
