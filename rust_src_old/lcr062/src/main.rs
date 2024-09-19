/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

fn main() {}

const LOWERCASE_SIZE: usize = 26;

struct Trie {
    root: TrieNode,
}

struct TrieNode {
    is_end: bool,
    next: [Option<Box<TrieNode>>; LOWERCASE_SIZE],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Trie {
            root: TrieNode {
                is_end: false,
                next: [
                    None, None, None, None, None, None, None, None, None, None, None, None, None,
                    None, None, None, None, None, None, None, None, None, None, None, None, None,
                ],
            },
        }
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut current_node = &mut self.root;

        for ch in word.chars() {
            let index = ch as usize - 'a' as usize;
            current_node = current_node.next[index].get_or_insert_with(|| {
                Box::new(TrieNode {
                    is_end: false,
                    next: [
                        None, None, None, None, None, None, None, None, None, None, None, None,
                        None, None, None, None, None, None, None, None, None, None, None, None,
                        None, None,
                    ],
                })
            })
        }

        current_node.is_end = true;
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let mut current_node = &self.root;

        for ch in word.chars() {
            let index = ch as usize - 97;
            if let Some(node) = &current_node.next[index] {
                current_node = node;
            } else {
                return false;
            }
        }

        current_node.is_end
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let mut current_node = &self.root;

        for ch in prefix.chars() {
            let index = ch as usize - 97;
            if let Some(node) = &current_node.next[index] {
                current_node = node;
            } else {
                return false;
            }
        }

        true
    }
}
