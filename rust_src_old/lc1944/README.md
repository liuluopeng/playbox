leetcode  rust解题模板



使用:
1. 安装generate:  `cargo install generate`
2. 使用模板:  
 `git clone this_repository`   
 `cargo generate this_template_path`



二维数组 输入: 复制leetcode的测试用例. 
```rust
let st = "[[1,2,3], [4,5,6]]";
let vec2d = leetcode_testcase_vec2d(st); 

// ===>
let vec2d = vec![
    vec![1,2,3],
    vec![4,5,6],
]
```

二叉树题目 输入: 
```rust
let st = "[1,2,3]";
let root = string2tree(st);
```


