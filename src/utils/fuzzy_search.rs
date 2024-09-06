use std::collections::HashMap;

#[derive(Clone)]
struct Node {
    children: HashMap<char, Node>,
    is_last: bool,
}

impl Node {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_last: false,
        }
    }
}

pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Self { root: Node::new() }
    }
    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;
        for ch in word.chars() {
            if !current_node.children.contains_key(&ch) {
                current_node.children.insert(ch, Node::new());
            }
            current_node = current_node.children.get_mut(&ch).unwrap();
        }
        current_node.is_last = true;
    }

    pub fn search(&mut self, word: &str) -> bool {
        let mut current_node = &mut self.root;
        for ch in word.chars() {
            if !current_node.children.contains_key(&ch) {
                return false;
            }
            current_node = current_node.children.get_mut(&ch).unwrap();
        }
        return current_node.is_last;
    }

    pub fn get_match(&self, word: &str) -> Vec<&str> {
        vec!["jako"]
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;
    #[test]
    fn search_for_word() {
        let mut t = Trie::new();
        t.insert("test");
        assert_eq!(t.search("te"), false);
        assert_eq!(t.search("test"), true);
    }
}
