

用0..n  0..n可以得到结果, 但是有重复的.
用start_i..n start_j有不能够到的结果.

2024年05月07日 02:20:44  
问了GPT4   棋盘有多少种放置的方法.他是一维展开的.

明天试试一维展开, 然后用start..n
2024年05月07日 12:06:54
可以了:

```rust
        // for i in 0..n {
        //     for j in 0..n {

        //         if board_now[i][j] == 0 {
        //             // println!("{:?} {:?}", i, j);
        //             board_now[i][j] = 1;

        //             Solution::find(boards, board_now, index + 1, i, j);
        //             // 恢复
        //             board_now[i][j] = 0;
        //         }

        //     }
        // }

        for inde_1d in start..n * n {
            let i = inde_1d / n;
            let j = inde_1d % n;

            if board_now[i][j] == 0 {
                // println!("{:?} {:?}", i, j);
                board_now[i][j] = 1;

                Solution::find(boards, board_now, index + 1, inde_1d, i, j);
                // 恢复
                board_now[i][j] = 0;
            }
        }
```