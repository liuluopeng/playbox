use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Node {
    key: i32,
    value: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32, value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            key,
            value,
            prev: None,
            next: None,
        }))
    }
}

struct LRUCache {
    capacity: usize,
    dummy_head: Rc<RefCell<Node>>,
    quick_finder: HashMap<i32, Rc<RefCell<Node>>>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let dummy = Node::new(0, 0);
        dummy.borrow_mut().prev = Some(dummy.clone());
        dummy.borrow_mut().next = Some(dummy.clone());
        LRUCache {
            capacity: capacity as usize,
            dummy_head: dummy,
            quick_finder: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.quick_finder.get(&key) {
            // 有这本书
            let node = node.clone();
            let value = node.borrow().value;
            self.remove(node.clone()); // 把这本书抽出来
            self.insert(node); // 放在最上面
            return value;
        }
        -1 // 没有这本书
    }

    fn get2(&mut self, key: i32) -> i32 {
        // 为了O(1), 需要一个map
        if let Some(n) = self.quick_finder.get(&key) {
            let node = n.clone();
            let n = n.clone();
            let n = n.borrow();
            let value = n.value;

            // println!("node: {:?} key: {} value {:?}", n, n.key, value);

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

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.quick_finder.get(&key) {
            // 有这本书
            let node = node.clone();
            node.borrow_mut().value = value; // 更新 value
            self.remove(node.clone()); // 把这本书抽出来
            self.insert(node); // 放在最上面
            return;
        }
        let node = Node::new(key, value); // 新书
        self.quick_finder.insert(key, node.clone());
        self.insert(node); // 放在最上面
        if self.quick_finder.len() > self.capacity {
            // 书太多了
            let back_node = self.dummy_head.borrow().prev.clone().unwrap();
            self.quick_finder.remove(&back_node.borrow().key);
            self.remove(back_node); // 去掉最后一本书
        }
    }

    // 删除一个节点（抽出一本书）
    // fn remove(&mut self, x: Rc<RefCell<Node>>) {
    //     let prev = x.borrow().prev.clone().unwrap();
    //     let next = x.borrow().next.clone().unwrap();
    //     prev.borrow_mut().next = Some(next.clone());
    //     next.borrow_mut().prev = Some(prev);
    // }

    fn remove(&mut self, node: Rc<RefCell<Node>>) {
        let prev_node = node.borrow().prev.clone().unwrap();
        let next_node = node.borrow().next.clone().unwrap();

        prev_node.borrow_mut().next = Some(next_node.clone());
        next_node.borrow_mut().prev = Some(prev_node);
    }

    // 在链表头添加一个节点（把一本书放在最上面）
    // fn insert(&mut self, x: Rc<RefCell<Node>>) {
    //     let next = self.dummy_head.borrow().next.clone();
    //     x.borrow_mut().prev = Some(self.dummy_head.clone());
    //     x.borrow_mut().next = next.clone();
    //     self.dummy_head.borrow_mut().next = Some(x.clone());
    //     next.unwrap().borrow_mut().prev = Some(x);
    // }
    // insert next to dummy head:
    fn insert(&mut self, node: Rc<RefCell<Node>>) {
        let next_node = self.dummy_head.borrow().next.clone();

        node.borrow_mut().prev = Some(self.dummy_head.clone());
        node.borrow_mut().next = next_node.clone();

        self.dummy_head.borrow_mut().next = Some(node.clone());
        next_node.unwrap().borrow_mut().prev = Some(node);
    }
}

// 作者：灵茶山艾府
// 链接：https://leetcode.cn/problems/lru-cache/solutions/2456294/tu-jie-yi-zhang-tu-miao-dong-lrupythonja-czgt/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处

fn main() {
    let mut obj = LRUCache::new(20);

    let ret_1 = obj.get(1);

    obj.put(1, 33);
    obj.put(2, 66);
    let ret_1 = obj.get(1);
}
