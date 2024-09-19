use std::{borrow::Borrow, thread::current};

struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn string_to_list_node(s: &str) -> Option<Box<ListNode>> {
    let s = s.trim_matches(|c| c == '[' || c == ']').replace(" ", "");
    let values: Vec<i32> = s.split(',').filter_map(|x| x.parse().ok()).collect();

    let mut head = None;
    let mut current = &mut head;

    for &val in values.iter() {
        *current = Some(Box::new(ListNode::new(val)));
        current = &mut current.as_mut().unwrap().next;
    }

    head
}

fn trav_list_node_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = head;

    while let Some(node) = current {
        result.push(node.val);
        current = node.next;
    }

    result
}

fn main() {
    println!("Hello, world!");
    let st = "[0,3,1,0,4,5,2,0]";
    let head = string_to_list_node(st);
    let res_head = Solution::merge_nodes(head);

    println!("{:?}", trav_list_node_to_vec(res_head));
}

impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res = vec![];

        // 第一个节点是0 所以从第二个节点开始:
        let mut current: Option<Box<ListNode>> = head.unwrap().next.clone();
        // let mut current: Option<Box<ListNode>> = head.and_then(|node| node.next);

        let mut sum = 0; // 已经累计的和
        while let Some(node) = current {
            
            if node.val == 0 {
                res.push(sum);
                sum = 0;
            } else {
                //
                sum += node.val
            }
            current = node.next;
        }

        let mut dummy = None;
        let mut current = &mut dummy;

        for &val in res.iter() {
            *current = Some(Box::new(ListNode::new(val)));
            current = &mut current.as_mut().unwrap().next;
        }

        dummy
    }
}
