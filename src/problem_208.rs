struct TrieByVec {
    is_end: bool,
    children: Vec<Option<Box<TrieByVec>>>,
}

impl TrieByVec {
    fn char_as_index(c: char) -> usize {
        (c as u8 - 'a' as u8) as usize
    }

    fn new() -> Self {
        let mut v = vec![];
        for _ in 0..26 {
            v.push(None);
        }
        TrieByVec {
            is_end: false,
            children: v,
        }
    }

    fn insert(&mut self, word: String) {
        let word: Vec<char> = word.chars().collect();
        let mut node = self;
        for c in word {
            let c_i = Self::char_as_index(c);
            let next = node.children.get_mut(c_i).unwrap();
            if next.is_none() {
                *next = Some(Box::new(Self::new()));
            }
            node = next.as_mut().unwrap();
        }
        node.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        let word: Vec<char> = word.chars().collect();
        let mut node = self;
        for c in word {
            let c_i = Self::char_as_index(c);
            match &node.children[c_i] {
                None => return false,
                Some(next) => node = &next,
            }
        }
        node.is_end
    }

    fn starts_with(&self, prefix: String) -> bool {
        let prefix: Vec<char> = prefix.chars().collect();
        let mut node = self;
        for c in prefix {
            let c_i = Self::char_as_index(c);
            match &node.children[c_i] {
                None => return false,
                Some(next) => node = &next,
            }
        }
        true
    }
}

use std::collections::HashMap;

struct TrieByMap {
    is_end: bool,
    children: HashMap<char, Box<TrieByMap>>,
}

impl TrieByMap {
    fn new() -> Self {
        TrieByMap {
            is_end: false,
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let word: Vec<char> = word.chars().collect();
        let mut node = self;
        for c in word {
            node = node.children
                .entry(c)
                .or_insert(Box::new(Self::new()));
        }
        node.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        let word: Vec<char> = word.chars().collect();
        let mut node = self;
        for c in word {
            match node.children.get(&c) {
                None => return false,
                Some(next) => node = &next,
            }
        }
        node.is_end
    }

    fn starts_with(&self, prefix: String) -> bool {
        let prefix: Vec<char> = prefix.chars().collect();
        let mut node = self;
        for c in prefix {
            match node.children.get(&c) {
                None => return false,
                Some(next) => node = &next,
            }
        }
        true
    }
}

#[test]
fn test_trie() {
    let mut trie = TrieByMap::new();

    trie.insert("apple".to_string());
    assert_eq!(trie.search("apple".to_string()), true);
    assert_eq!(trie.search("app".to_string()), false);
    assert_eq!(trie.starts_with("app".to_string()), true);
    trie.insert("app".to_string());
    assert_eq!(trie.search("app".to_string()), true);
}