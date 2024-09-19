fn main() {
    println!("Hello, world!");

    let mut arr = vec![15, 6, 15, 7, 16, 8, 7, 0];

    left_first_less(&arr);

    println!();
    right_first_less_rev(&arr);

    println!();

    // right_first_less(&arr);

    // println!();

    // right_first_more(&arr);
    // println!();
}

fn left_first_less(arr: &Vec<usize>) {
    let mut stack = vec![];

    for (i, &v) in arr.iter().enumerate() {
        while !stack.is_empty() && arr[stack[stack.len() - 1]] >= v {
            stack.pop();
        }

        if !stack.is_empty() {
            // 存在一个左边的较小数字
            println!(
                "{} 的左边较小的数字是  {}",
                arr[i],
                arr[stack[stack.len() - 1]]
            );
        } else {
            println!("{}    的左边没有较小的数字    ", arr[i]);
        }

        stack.push(i);
    }
}
fn right_first_less_rev(arr: &Vec<usize>) {
    let mut stack = vec![];

    for (i, v) in arr.iter().enumerate().rev() {
        while !stack.is_empty() && arr[i] <= arr[stack[stack.len() - 1]] {
            stack.pop();
        }

        if !stack.is_empty() {
            println!(
                "{} 的右边较小的数字是  {}",
                arr[i],
                arr[stack[stack.len() - 1]]
            );
        } else {
            println!("{}    的右边没有较小的数字    ", arr[i]);
        }

        stack.push(i);
    }
}



// 2024年6月19日15:44:29  看不懂 这个 : 
fn right_first_less(arr: &Vec<usize>) {
    let mut stack = vec![];

    for (i, v) in arr.iter().enumerate() {
        let mut setted_flag = false;
        let mut p = arr.len();

        while !stack.is_empty() && arr[stack[stack.len() - 1]] <= arr[i] {
            p = stack.pop().unwrap();
            setted_flag = true;
        }

        if setted_flag == false {
            println!("{}    的右边没有较小的数字    ", arr[i]);
        } else {
            println!("{}  的右边第一个较小的数字:   {}", arr[i], arr[p]);
        }

        stack.push(i);
    }
}
fn right_first_more(arr: &Vec<usize>) {
    let mut stack = vec![];

    for (i, v) in arr.iter().enumerate() {
        let mut setted_flag = false;
        let mut p = arr.len();

        while !stack.is_empty() && arr[stack[stack.len() - 1]] < arr[i] {
            p = stack.pop().unwrap();
            setted_flag = true;
        }

        if setted_flag == false {
            println!("{}    的右边没有较大的数字    ", arr[i]);
        } else {
            println!("{}  的右边第一个较大的数字:   {}", arr[i], arr[p]);
        }

        stack.push(i);
    }
}
