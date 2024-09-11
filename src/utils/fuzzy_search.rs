use std::collections::HashMap;

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
            if !current_node.children.contains_key(&ch) {
                current_node.children.insert(ch, Node::new());
            }
            current_node = current_node.children.get_mut(&ch).unwrap();
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
        return current_node.is_last;
    }

    pub fn autocomplete(&mut self, prefix: &str) -> Vec<&str> {
        let mut current_node = &mut self.root;
        //

        vec!["test"]
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

        // vec!["jako"].binary_search_by(|x| x.ke);
        // let ok = |x| {
        //     if ok(x) {
        //         return Ordering::Greater;
        //     }
        //     Ordering::Less
        // };
        let test = vec!["ako"];
        // test.pop()
        // vec!["jkao"].cmp()
        assert_eq!(1.cmp(&2), Ordering::Less);
        let jako = vec!["jako"].binary_search(&"jako").unwrap();
        assert_eq!(jako, 0);
    }
}
