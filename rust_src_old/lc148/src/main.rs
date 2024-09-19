use leetcode::{linked_list::ListNode, solution::Solution, util::read_numbers_from_file};

fn main() {
    println!("Hello, world!");

    let arr = read_numbers_from_file("testcase1.txt");

    match arr {
        Ok(numbers) => {
            // println!("{:?}", numbers);

            let head = ListNode::load_from_testcase(numbers);

            // println!("load res: {:?}", head.clone());

            let head = Solution::sort_list(head);

            head.unwrap().peek();
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
