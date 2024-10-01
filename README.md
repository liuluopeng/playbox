


# 我的 数据结构  算法  仓库

包含:   
- leetcode模板.
- mdbook生成的电子书
- rust的 常见数据结构  经典算法题   leetcode题目题解



## leetcode  rust解题模板

使用:
1. 安装generate:  `cargo install generate`
2. 使用模板:  
 `git clone this_repository`   
 `cargo generate this_template_path`



### 开始解题的步骤:

在`src/leetcode/mod.rs` 声明新建的文件:  
`pub mod lc1`  

创建`lc1.rs`, 然后粘贴函数的格式:  

```rust
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    }
}
```


然后复制这段代码(或者更加快捷的方式:使用vs code -> 首选项 -> 配置代码片段):   

```rust

struct Solution{}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}

```





### 快捷方式  

二维数组 输入: 复制leetcode的测试用例. 
```rust
pub fn array2d_to_vec2d<T, const N: usize>(array: &[T; N]) -> Vec<Vec<i32>>
where
    T: AsRef<[i32]>, // 每一行都可以被引用为 i32 数组
{
    array
        .iter()
        .map(|row| row.as_ref().to_vec()) // 将每一行转换为 Vec<i32>
        .collect() // 收集为 Vec<Vec<i32>>
}
```

二叉树题目 输入: 
```rust
let st = "[1,2,3]";
let root = string2tree(st);
```

## mdbook 相关
使用了的插件:  
- markdown 流程图 rust插件
- 数学符号插件 mdbook-katex 


右侧的缩略目录:  `index.hbs` 的 `pagetoc` 






