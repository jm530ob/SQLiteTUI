use std::collections::HashMap;

#[derive(Clone, Default)]
struct Node {
    children: HashMap<char, Node>,
    // value: Option<String>,
    is_last: bool,
}

impl Node {
    fn new() -> Self {
        Self {
            ..Default::default()
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
            current_node = current_node.children.entry(ch).or_default();
        }
        // current_node.value.replace(word.to_owned());
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

    pub fn autocomplete(&mut self, prefix: &str) -> Vec<String> {
        let mut current_node = &mut self.root;

        for ch in prefix.chars() {
            if let Some(node) = current_node.children.get_mut(&ch) {
                current_node = node;
            } else {
                return vec![];
            }
        }

        let mut list: Vec<String> = Vec::new();
        Self::traverse_tree(&current_node, &mut list, prefix);

        list
    }

    // Add doc
    fn traverse_tree(node: &Node, auto_comp_words: &mut Vec<String>, prefix: &str) {
        if node.children.is_empty() {
            return;
        }
        if node.is_last {
            auto_comp_words.push(prefix.to_string());
        }
        let map = &node.children;
        for (ch, n) in map {
            Self::traverse_tree(n, auto_comp_words, &format!("{prefix}{ch}"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;
    #[test]
    #[ignore]
    fn search_for_word() {
        let mut t = Trie::new();
        t.insert("test");
        assert_eq!(t.search("te"), false);
        assert_eq!(t.search("test"), true);

        t.insert("foo");
        t.insert("bar");
        t.insert("bazz");

        assert_eq!(t.autocomplete("ba"), vec!["bar", "bazz"]); // this order may change, potentionaly cause test to fail
        assert_eq!(t.autocomplete("bar"), vec!["bar"]);
        assert_eq!(t.autocomplete("bazzer"), Vec::<String>::new());
    }
}
