use std::collections::HashMap;

use tokio::sync::OwnedRwLockMappedWriteGuard;

#[derive(Clone, Default)]
struct Node {
    children: HashMap<char, Node>,
    value: Option<String>,
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
        current_node.value.replace(word.to_owned());
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
        // for ch in word.chars() {
        //     match current_node.children.get(&ch) {
        //         Some(_) => current_node = current_node.children.get_mut(&ch).unwrap(),
        //         None => return false,
        //     };
        // }
        return current_node.is_last;
    }

    fn recursive_search(node: &Node) -> Option<&str> {
        if node.is_last {
            return Some(node.value.as_deref().unwrap());
        } else {
            for (_, n) in &node.children {
                Self::recursive_search(&n); // to je zle celkom
            }
            None // dummy check
        }
    }

    pub fn autocomplete(&mut self, prefix: &str) -> Vec<&str> {
        let mut current_node = &mut self.root;

        for ch in prefix.chars() {
            // if !current_node.children.contains_key(&ch) {
            //     return vec![];
            // }
            // current_node = current_node.children.get_mut(&ch).unwrap();
            if let Some(node) = current_node.children.get_mut(&ch) {
                current_node = node;
            } else {
                return vec![];
            }
        }

        // let recursive_search = |n| {};
        let active_words = current_node
            .children
            .iter()
            .map(|(_, n)| Self::recursive_search(n).unwrap())
            .collect::<Vec<&str>>();

        active_words
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::Trie;
    #[test]
    fn search_for_word() {
        let mut t = Trie::new();
        t.insert("test");
        assert_eq!(t.search("te"), false);
        assert_eq!(t.search("test"), true);

        t.insert("foo");
        t.insert("bar");
        t.insert("bazz");

        assert_eq!(t.autocomplete("ba"), vec!["bar, bazz"]);
        assert_eq!(t.autocomplete("bar"), vec!["bar"]);

        assert_eq!(1.cmp(&2), Ordering::Less);
        let jako = vec!["jako"].binary_search(&"jako").unwrap();
        assert_eq!(jako, 0);
    }
}
