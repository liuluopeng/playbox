use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    key: i32,
    value: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(key: i32, value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            key,
            value,
            prev: None,
            next: None,
        }))
    }
}

#[derive(Debug)]
struct LRUCache {
    capacity: usize,
    dummy_head: Rc<RefCell<Node>>,
    quick_finder: HashMap<i32, Rc<RefCell<Node>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        // let dummy_head = Rc::new(RefCell::new(Node::new(-1, -1)));
        let dummy_head = Node::new(-1, -1);
        dummy_head.borrow_mut().next = Some(dummy_head.clone());
        dummy_head.borrow_mut().prev = Some(dummy_head.clone());

        let capacity = capacity as usize;

        LRUCache {
            capacity: capacity,
            dummy_head: dummy_head,
            quick_finder: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        // 为了O(1), 需要一个map
        if let Some(n) = self.quick_finder.get(&key) {
            let node = n.clone();
            // let refmut = node2.borrow();   回环的问题所在.不能单独弄成变量.
            let value = node.borrow().value;

            // 放到dummy后面
            // 先删除它
            self.remove(node.clone());
            // 然后 重新插入 它
            self.insert(node);

            return value;
        } else {
            return -1;
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        // 检查 是否存在key
        if let Some(n) = self.quick_finder.get(&key) {
            let node = n.clone();
            let n = n.clone();
            // let mut n = n.borrow_mut(); // 这里还是一样,borrow error
            // n.value = value; // 修改成新的
            n.borrow_mut().value = value;
            // 移动到最前面
            self.remove(node.clone());
            self.insert(node);
        } else {
            // let new_node = Rc::new(RefCell::new(Node::new(key, value)));
            let new_node = Node::new(key, value);
            self.quick_finder.insert(key, new_node.clone());
            // 移动到最前面:
            self.insert(new_node.clone());

            if self.quick_finder.len() > self.capacity {
                let last_node = self.dummy_head.borrow().prev.clone().unwrap();
                // 及时去掉map里面的结点
                self.quick_finder.remove(&last_node.borrow().key);
                self.remove(last_node);
            }
        }

        // 放到dummy后 dummy后是最上面
    }

    /*

           dymmy <->  node1 <-> node2  <-> node_capacity

    */
    fn remove(&mut self, node: Rc<RefCell<Node>>) {
        let prev_node = node.borrow().prev.clone().unwrap();
        let next_node = node.borrow().next.clone().unwrap();

        prev_node.borrow_mut().next = Some(next_node.clone());
        next_node.borrow_mut().prev = Some(prev_node);
    }

    // insert next to dummy head:
    fn insert(&mut self, node: Rc<RefCell<Node>>) {
        let next_node = self.dummy_head.borrow().next.clone();

        node.borrow_mut().prev = Some(self.dummy_head.clone());
        node.borrow_mut().next = next_node.clone();

        self.dummy_head.borrow_mut().next = Some(node.clone());
        next_node.unwrap().borrow_mut().prev = Some(node);
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

fn main() {
    let mut obj = LRUCache::new(20);

    let ret_1 = obj.get(1);

    obj.put(1, 33);
    obj.put(2, 66);
    let ret_1 = obj.get(1);
}
