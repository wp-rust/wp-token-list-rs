use std::collections::HashSet;
use std::fmt;

/// A set of tokens, similar to DOMTokenList.
#[derive(Debug, Clone, Default)]
pub struct TokenList {
    value_as_array: Vec<String>,
}

impl TokenList {
    /// Constructs a new instance of TokenList.
    pub fn new(initial_value: &str) -> Self {
        let mut list = Self {
            value_as_array: Vec::new(),
        };
        list.set_value(initial_value);
        list
    }

    /// Returns the associated set as a string.
    pub fn value(&self) -> String {
        self.value_as_array.join(" ")
    }

    /// Replaces the associated set with a new string value.
    pub fn set_value(&mut self, value: &str) {
        let mut seen = HashSet::new();
        self.value_as_array = value
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .filter_map(|s| {
                if seen.insert(s.to_string()) {
                    Some(s.to_string())
                } else {
                    None
                }
            })
            .collect();
    }

    /// Returns the number of tokens.
    pub fn length(&self) -> usize {
        self.value_as_array.len()
    }

    /// Returns true if the token list is empty.
    pub fn is_empty(&self) -> bool {
        self.value_as_array.is_empty()
    }

    /// Returns the token with the given index.
    pub fn item(&self, index: usize) -> Option<&String> {
        self.value_as_array.get(index)
    }

    /// Returns true if `token` is present, and false otherwise.
    pub fn contains(&self, item: &str) -> bool {
        self.value_as_array.iter().any(|val| val == item)
    }

    /// Adds all arguments passed, except those already present.
    pub fn add(&mut self, items: &[&str]) {
        for &item in items {
            if !self.contains(item) {
                self.value_as_array.push(item.to_string());
            }
        }
    }

    /// Removes arguments passed, if they are present.
    pub fn remove(&mut self, items: &[&str]) {
        self.value_as_array.retain(|val| !items.contains(&val.as_str()));
    }

    /// If `force` is not given, "toggles" `token`, removing it if it's present
    /// and adding it if it's not present. If `force` is true, adds token (same
    /// as add()). If force is false, removes token (same as remove()). Returns
    /// true if `token` is now present, and false otherwise.
    pub fn toggle(&mut self, token: &str, force: Option<bool>) -> bool {
        let should_add = match force {
            Some(f) => f,
            None => !self.contains(token),
        };

        if should_add {
            self.add(&[token]);
        } else {
            self.remove(&[token]);
        }

        should_add
    }

    /// Replaces `token` with `new_token`. Returns true if `token` was replaced
    /// with `new_token`, and false otherwise.
    pub fn replace(&mut self, token: &str, new_token: &str) -> bool {
        if !self.contains(token) {
            return false;
        }

        self.remove(&[token]);
        self.add(&[new_token]);

        true
    }

    /// Returns true if `token` is in the associated attribute's supported
    /// tokens. Returns false otherwise. Always returns `true` in this implementation.
    pub fn supports(&self, _token: &str) -> bool {
        true
    }

    /// Returns an iterator over the tokens.
    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.value_as_array.iter()
    }
}

impl fmt::Display for TokenList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl IntoIterator for TokenList {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.value_as_array.into_iter()
    }
}

impl<'a> IntoIterator for &'a TokenList {
    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let list = TokenList::new("a b c");
        assert_eq!(list.value(), "a b c");
        assert_eq!(list.length(), 3);
    }

    #[test]
    fn test_new_with_duplicates_and_spaces() {
        let list = TokenList::new("  a  b   a c  ");
        assert_eq!(list.value(), "a b c");
        assert_eq!(list.length(), 3);
    }

    #[test]
    fn test_set_value() {
        let mut list = TokenList::new("a");
        list.set_value("b c b");
        assert_eq!(list.value(), "b c");
    }

    #[test]
    fn test_item() {
        let list = TokenList::new("a b c");
        assert_eq!(list.item(0).map(|s| s.as_str()), Some("a"));
        assert_eq!(list.item(2).map(|s| s.as_str()), Some("c"));
        assert_eq!(list.item(3), None);
    }

    #[test]
    fn test_contains() {
        let list = TokenList::new("a b");
        assert!(list.contains("a"));
        assert!(!list.contains("c"));
    }

    #[test]
    fn test_add() {
        let mut list = TokenList::new("a");
        list.add(&["b", "c"]);
        assert_eq!(list.value(), "a b c");

        // Adding existing does nothing
        list.add(&["a", "d"]);
        assert_eq!(list.value(), "a b c d");
    }

    #[test]
    fn test_remove() {
        let mut list = TokenList::new("a b c d");
        list.remove(&["b", "d", "e"]);
        assert_eq!(list.value(), "a c");
    }

    #[test]
    fn test_toggle() {
        let mut list = TokenList::new("a b");
        
        // toggle existing without force removes it
        assert_eq!(list.toggle("b", None), false);
        assert_eq!(list.value(), "a");

        // toggle non-existing without force adds it
        assert_eq!(list.toggle("c", None), true);
        assert_eq!(list.value(), "a c");

        // toggle with force true adds it
        assert_eq!(list.toggle("d", Some(true)), true);
        assert_eq!(list.value(), "a c d");

        // toggle with force false removes it
        assert_eq!(list.toggle("a", Some(false)), false);
        assert_eq!(list.value(), "c d");
    }

    #[test]
    fn test_replace() {
        let mut list = TokenList::new("a b c");
        assert_eq!(list.replace("b", "d"), true);
        assert_eq!(list.value(), "a c d");

        assert_eq!(list.replace("e", "f"), false);
        assert_eq!(list.value(), "a c d");
    }

    #[test]
    fn test_supports() {
        let list = TokenList::new("a");
        assert!(list.supports("anything"));
    }

    #[test]
    fn test_display() {
        let list = TokenList::new("a b");
        assert_eq!(list.to_string(), "a b");
    }

    #[test]
    fn test_iter() {
        let list = TokenList::new("a b");
        let vec: Vec<_> = list.iter().map(|s| s.as_str()).collect();
        assert_eq!(vec, vec!["a", "b"]);
    }
}
