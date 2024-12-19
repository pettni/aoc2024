use crate::hash::*;

#[derive(Debug)]
pub struct TrieNode {
    is_end: bool,
    children: FxHashMap<char, TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            is_end: false,
            children: FxHashMap::new(),
        }
    }

    pub fn is_end(&self) -> bool {
        self.is_end
    }

    pub fn step(&self, c: char) -> Option<&TrieNode> {
        self.children.get(&c)
    }
}

#[derive(Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Default for Trie {
    fn default() -> Trie {
        Trie {
            root: TrieNode::new(),
        }
    }
}

impl Trie {
    /// Create a new trie.
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    /// Create a new trie from words.
    pub fn from_words(words: &[&str]) -> Self {
        let mut ret = Trie::new();
        for word in words {
            ret.insert(word);
        }
        ret
    }

    /// Insert a new word into trie.
    pub fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;
        for c in word.chars() {
            current = current.children.entry(c).or_insert(TrieNode::new());
        }
        current.is_end = true;
    }

    /// Check if 'word' contained in trie.
    pub fn search(&mut self, word: &str) -> bool {
        let mut current = &mut self.root;

        for c in word.chars() {
            match current.children.get_mut(&c) {
                Some(node) => current = node,
                None => return false,
            }
        }

        current.is_end
    }

    /// Check if any word starts with 'prefix'.
    pub fn prefix(&mut self, prefix: &str) -> bool {
        let mut current = &self.root;

        for c in prefix.chars() {
            match current.children.get(&c) {
                Some(node) => current = node,
                None => return false,
            }
        }

        true
    }

    /// Return root node
    pub fn root(&self) -> &TrieNode {
        &self.root
    }

    /// Step into trie.
    pub fn step(&self, c: char) -> Option<&TrieNode> {
        self.root.step(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let mut trie = Trie::default();
        trie.insert("hello");
        trie.insert("helios");
        assert!(!trie.search("he"));
        assert!(trie.search("hello"));
        assert!(trie.search("helios"));
        assert!(!trie.search("hello world"));
    }

    #[test]
    fn test_trie_step() {
        let mut trie = Trie::default();
        trie.insert("hello");
        trie.insert("helios");
        trie.insert("havana");
        trie.insert("hel");

        assert!(trie.step('x').is_none());
        let h = trie.step('h').unwrap();
        let e = h.step('e').unwrap();
        let l = e.step('l').unwrap();
        assert!(l.is_end());
        let l = l.step('l').unwrap();
        let o = l.step('o').unwrap();
        assert!(o.is_end());
        assert!(o.step('o').is_none());
    }
}
