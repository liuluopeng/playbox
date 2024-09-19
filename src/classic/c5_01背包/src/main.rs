fn main() {
    println!("Hello, world!");

    let mut value_list = vec![6, 10, 12];
    let mut weight_list = vec![1, 2, 3];
    let mut capacity = 5;
}

fn max_value_when_index(
    value_list: &Vec<i32>,
    weight_list: &Vec<i32>,
    index: usize,
    remain_capacity: i32,
) -> i32 {
    if index == 0 {

    }
    
    if remain_capacity <= 0 {
        return 0;
    }

    let mut res = max_value_when_index(
        value_list,
        weight_list,
        index - 1,
        remain_capacity, // 把余量给前面的
    );

    if weight_list[index] < remain_capacity {
        // 把余量给自己
        res = res.max(
            value_list[index]
                + max_value_when_index(
                    value_list,
                    weight_list,
                    index - 1,
                    remain_capacity - weight_list[index],
                ),
        );
    }

    res
}
