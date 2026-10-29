#[derive(Debug)]
pub struct Trie {
    node_count: usize,
    children: Vec<Vec<(char, usize)>>,
    is_end: Vec<bool>,
}

impl Trie {
    const ROOT: usize = 0;
    pub fn new() -> Trie {
        Trie {
            node_count: 1,
            children: vec![Vec::new(); 1],
            is_end: vec![false; 1],
        }
    }

    pub fn add_word(&mut self, word: &str) {
        let mut node = Trie::ROOT;
        for c in word.chars() {
            node = self.next_node_or_add(node, c)
        }
        self.is_end[node] = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut node = Trie::ROOT;
        for c in word.chars() {
            if let Some(next_node) = self.next_node(node, c) {
                node = next_node;
            } else {
                return false;
            }
        }
        self.is_end[node]
    }

    fn next_node(&self, node: usize, c: char) -> Option<usize> {
        self.children[node]
            .iter()
            .find(|(label, _)| *label == c)
            .map(|(_, child)| *child)
    }

    fn next_node_or_add(&mut self, node: usize, c: char) -> usize {
        if let Some(child) = self.next_node(node, c) {
            child
        } else {
            let new_node = self.node_count;
            self.children.push(Vec::new());
            self.is_end.push(false);
            self.children[node].push((c, new_node));
            self.node_count += 1;
            new_node
        }
    }
}

impl From<Vec<&str>> for Trie {
    fn from(words: Vec<&str>) -> Self {
        let mut root = Trie::new();
        for word in words {
            root.add_word(word);
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_trie() {
        let trie = Trie::new();
        assert_eq!(trie.node_count, 1);
        assert_eq!(trie.children.len(), 1);
    }

    #[test]
    fn test_add_single_word() {
        let mut trie = Trie::new();
        trie.add_word("hello");
        assert!(trie.contains("hello"));
    }

    #[test]
    fn test_add_multiple_words() {
        let mut trie = Trie::new();
        trie.add_word("cat");
        trie.add_word("car");
        trie.add_word("dog");
        assert!(trie.contains("cat"));
        assert!(trie.contains("car"));
        assert!(trie.contains("dog"));
    }

    #[test]
    fn test_contains_nonexistent_word() {
        let mut trie = Trie::new();
        trie.add_word("hello");
        assert!(!trie.contains("hell"));
        assert!(!trie.contains("helloworld"));
        assert!(!trie.contains("world"));
    }

    #[test]
    fn test_from_vec() {
        let trie = Trie::from(vec!["apple", "app", "apply"]);
        assert!(trie.contains("apple"));
        assert!(trie.contains("app"));
        assert!(trie.contains("apply"));
        assert!(!trie.contains("ap"));
    }

    #[test]
    fn test_shared_prefixes() {
        let mut trie = Trie::new();
        trie.add_word("test");
        trie.add_word("testing");
        trie.add_word("tester");
        assert!(trie.contains("test"));
        assert!(trie.contains("testing"));
        assert!(trie.contains("tester"));
        assert!(!trie.contains("tes"));
    }

    #[test]
    fn test_empty_word() {
        let mut trie = Trie::new();
        trie.add_word("");
        assert!(trie.contains(""));
    }

    #[test]
    fn test_single_char_words() {
        let mut trie = Trie::new();
        trie.add_word("a");
        trie.add_word("b");
        trie.add_word("c");
        assert!(trie.contains("a"));
        assert!(trie.contains("b"));
        assert!(trie.contains("c"));
        assert!(!trie.contains("d"));
    }

    #[test]
    fn test_unicode_support() {
        let mut trie = Trie::new();
        trie.add_word("こんにちは");
        trie.add_word("世界");
        assert!(trie.contains("こんにちは"));
        assert!(trie.contains("世界"));
        assert!(!trie.contains("こん"));
    }
}
