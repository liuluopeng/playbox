use std::collections::HashMap;

struct MapSum {
    m: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MapSum {
    fn new() -> Self {
        MapSum { m: HashMap::new() }
    }

    fn insert(&mut self, key: String, val: i32) {
        if self.m.contains_key(&key) {
            self.m.insert(key, val);
        } else {
            self.m.insert(key, val);
        }
    }

    // ["MapSum","insert","sum","insert","sum"]
    // [[],["apple",3],["ap"],["app",2],["ap"]]

    fn sum(&self, prefix: String) -> i32 {
        let mut res = 0;
        for (key, value) in &self.m {
            if key.starts_with(&prefix) {
                res += value;
            }
        }

        res
    }
}

/**
 * Your MapSum object will be instantiated and called as such:
 * let obj = MapSum::new();
 * obj.insert(key, val);
 * let ret_2: i32 = obj.sum(prefix);
 */

fn main() {
    println!("Hello, world!");
}
