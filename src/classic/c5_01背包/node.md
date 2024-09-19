```rust

fn knapsack(weights: &[u32], values: &[u32], capacity: u32) -> u32 {
    let n = weights.len();
    let mut dp = vec![vec![0; (capacity + 1) as usize]; n + 1];

    for i in 1..=n {
        for w in 1..=capacity {
            if weights[i - 1] <= w {
                dp[i][w as usize] = dp[i - 1][w as usize].max(values[i - 1] + dp[i - 1][(w - weights[i - 1]) as usize]);
            } else {
                dp[i][w as usize] = dp[i - 1][w as usize];
            }
        }
    }

    dp[n][capacity as usize]
}

fn main() {
    let weights = vec![2, 3, 4, 5];
    let values = vec![3, 4, 5, 6];
    let capacity = 8;

    let max_value = knapsack(&weights, &values, capacity);
    println!("The maximum value that can be achieved is: {}", max_value);
}

```


```rust
fn knapsack(weights: &[u32], values: &[u32], capacity: u32) -> (u32, Vec<usize>) {
    let n = weights.len();
    let mut dp = vec![vec![0; (capacity + 1) as usize]; n + 1];
    let mut selected_items = vec![0; n];

    for i in 1..=n {
        for w in 1..=capacity {
            if weights[i - 1] <= w {
                let take = values[i - 1] + dp[i - 1][(w - weights[i - 1]) as usize];
                let not_take = dp[i - 1][w as usize];

                if take >= not_take {
                    dp[i][w as usize] = take;
                    selected_items[i - 1] = 1;
                } else {
                    dp[i][w as usize] = not_take;
                }
            } else {
                dp[i][w as usize] = dp[i - 1][w as usize];
            }
        }
    }

    let mut max_value = dp[n][capacity as usize];
    let mut selected_indices = Vec::new();
    let mut remaining_capacity = capacity as i32;

    for i in (0..n).rev() {
        if selected_items[i] == 1 {
            selected_indices.push(i);
            remaining_capacity -= weights[i] as i32;
        }
    }

    (max_value, selected_indices)
}

fn main() {
    let weights = vec![2, 3, 4, 5];
    let values = vec![3, 4, 5, 6];
    let capacity = 8;

    let (max_value, selected_indices) = knapsack(&weights, &values, capacity);
    println!("The maximum value that can be achieved is: {}", max_value);
    println!("Selected items indices: {:?}", selected_indices);
}

```