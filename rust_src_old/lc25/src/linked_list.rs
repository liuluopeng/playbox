// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    /// 看一眼链表
    pub fn peek(&self) {
        print!("{}", self.val);

        if let Some(node) = &self.next {
            print!(" -> ");
            node.peek();
        } else {
            print!("\n");
        }
    }

    /// 从测试用例装载链表数据 [1,2,3,4,5]
    pub fn load_from_testcase(arr: Vec<i32>) -> Option<Box<Self>> {
        let mut head = None;
        let mut current = &mut head;

        for num in arr {
            *current = Some(Box::new(ListNode::new(num)));
            current = &mut current.as_mut().unwrap().next;
        }

        head
    }
}
