struct WordDictionary {
    is_end: bool,
    children: Vec<Option<Box<WordDictionary>>>,
}

impl WordDictionary {
    fn char_as_index(c: char) -> usize {
        (c as u8 - 'a' as u8) as usize
    }

    fn new() -> Self {
        let mut v = vec![];
        for _ in 0..26 {
            v.push(None);
        }
        Self {
            is_end: false,
            children: v,
        }
    }

    fn add_word(&mut self, word: String) {
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

        fn core(node: &WordDictionary, word: &[char]) -> bool {
            if word.len() == 0 {
                node.is_end
            } else {
                let c = word[0];
                if c == '.' {
                    for i in 0..26 {
                        match &node.children[i] {
                            None => continue,
                            Some(next) => if core(&next, &word[1..]) {
                                return true;
                            },
                        }
                    }
                    false
                } else {
                    let c_i = WordDictionary::char_as_index(c);
                    match &node.children[c_i] {
                        None => false,
                        Some(next) => core(&next, &word[1..]),
                    }
                }
            }
        }

        core(self, word.as_slice())
    }
}