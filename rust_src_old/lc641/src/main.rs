// use std::borrow::BorrowMut;
// use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq)]
struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>,
}
impl Node {
    fn new(val: i32) -> Self {
        Node {
            val: val,
            next: None,
            prev: None,
        }
    }
}
struct MyCircularDeque {
    dummy_head: Rc<RefCell<Node>>,
    capacity: usize,
    size: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        let dummy_head = Rc::new(RefCell::new(Node::new(-1)));

        dummy_head.borrow_mut().next = Some(dummy_head.clone());
        dummy_head.borrow_mut().prev = Some(dummy_head.clone());

        MyCircularDeque {
            dummy_head: dummy_head,
            capacity: k as usize,
            size: 0,
        }
    }

    //   dummy <-> node1  <-> node2

    fn insert_front(&mut self, value: i32) -> bool {
        if self.size == self.capacity {
            return false;
        }

        let new_node = Rc::new(RefCell::new(Node::new(value)));

        let ori_head = self.dummy_head.borrow().next.clone();

        new_node.borrow_mut().next = ori_head.clone();
        new_node.borrow_mut().prev = Some(self.dummy_head.clone());

        self.dummy_head.borrow_mut().next = Some(new_node.clone());
        ori_head.unwrap().borrow_mut().prev = Some(new_node);

        self.size += 1;

        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.size == self.capacity {
            return false;
        }

        // println!("what insert: {:?}  ", value);
        // self.show();

        let new_node = Rc::new(RefCell::new(Node::new(value)));

        let ori_tail = self.dummy_head.borrow().prev.clone();

        new_node.borrow_mut().next = Some(self.dummy_head.clone());
        new_node.borrow_mut().prev = ori_tail.clone();

        self.dummy_head.borrow_mut().prev = Some(new_node.clone());
        ori_tail.unwrap().borrow_mut().next = Some(new_node);

        self.size += 1;
        true
    }
    //   dummy <-> node1  <-> node2

    fn delete_front(&mut self) -> bool {
        if self.size == 0 {
            return false;
        }

        let second_node = self
            .dummy_head
            .borrow()
            .next
            .clone()
            .unwrap()
            .borrow()
            .next
            .clone();
        self.dummy_head.borrow_mut().next = second_node.clone();
        second_node.unwrap().borrow_mut().prev = Some(self.dummy_head.clone());
        self.size -= 1;
        true
    }

    fn delete_last(&mut self) -> bool {
        if self.size == 0 {
            return false;
        }

        let last_2nd_node = self
            .dummy_head
            .borrow()
            .prev
            .clone()
            .unwrap()
            .borrow()
            .prev
            .clone();

        self.dummy_head.borrow_mut().prev = last_2nd_node.clone();
        last_2nd_node.unwrap().borrow_mut().next = Some(self.dummy_head.clone());
        self.size -= 1;
        true
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }

        let f = self.dummy_head.borrow().next.clone();
        let f = f.as_ref();
        if let Some(n) = f {
            let n = n.borrow();
            return n.val;
        } else {
            return -1;
        }
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }

        return self.dummy_head.borrow().prev.clone().unwrap().borrow().val;
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size >= self.capacity
    }

    fn show(&self) {
        let over = self.dummy_head.clone();

        let mut current_node = self.dummy_head.borrow().next.clone();

        while let Some(node) = current_node.clone() {
            // if over == node {
            //     break;
            // }

            let node = node.borrow();

            if node.val == -1 {
                break;
            }

            println!("{:?} ->", node.val);

            current_node = node.next.clone();
        }
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */
fn main() {
    println!("Hello, world!");
    let k = 20;

    let value = 3;

    let mut obj = MyCircularDeque::new(k);
    let ret_1: bool = obj.insert_front(7);
    let ret_2: bool = obj.insert_last(0);
    let ret_2: bool = obj.insert_last(3);
    obj.insert_front(9);

    // let ret_3: bool = obj.delete_front();

    obj.show();
}
