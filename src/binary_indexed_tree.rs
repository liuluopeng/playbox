pub struct BinaryIndexedTree {
    // data: Vec<i32>,
    tree: Vec<i32>,
}

// 前缀和
impl BinaryIndexedTree {
    pub fn new(data: Vec<i32>) -> Self {
        let tree = vec![0; data.len()];
        let mut my_tree = BinaryIndexedTree { tree: tree };

        for (idx, &value) in data.iter().enumerate() {
            my_tree.add(idx, value);
        }

        my_tree
    }

    // 最右测的1的位置
    fn low1_position(idx: usize) -> usize {
        /*
           0 0 1 1 0
           1 1 0 1 0
          &_________
           0 0 0 1 0

           result: 2  确实如此
        */

        idx & (!idx + 1)
    }

    // 操作: data[i]的数值  添加x
    pub fn add(&mut self, idx: usize, add: i32) {
        let mut base1_idx = idx + 1;

        // 影响的是idx右侧的一些下标
        while base1_idx <= self.tree.len() {
            self.tree[base1_idx - 1] += add;

            base1_idx += BinaryIndexedTree::low1_position(base1_idx);
        }
    }

    // 操作: 查询[0..idx]的范围和
    pub fn query(&self, idx: usize) -> i32 {
        let mut base1_idx = idx + 1;

        let mut sum = 0;

        while base1_idx >= 1 {
            // 寻找tree[base1_idx]的表达的范围 的 公式:    ~ (base1_idx)
            sum += self.tree[base1_idx - 1];
            base1_idx = (base1_idx - BinaryIndexedTree::low1_position(base1_idx) + 1) - 1;
        }

        sum
    }

    // 表示范围:
    fn rep(base1_idx: usize) {
        let base1_start = base1_idx - BinaryIndexedTree::low1_position(base1_idx) + 1;
        let base1_end = base1_idx;

        println!(
            "{:?} 表示的范围: {:?} ~ {:?}",
            base1_idx, base1_start, base1_end
        );
    }

    /// 操作: 查询[l..r]的范围和
    pub fn query_range(&self, l: usize, r: usize) -> i32 {
        if l == 0 {
            return Self::query(&self, r);
        }
        //  l-1  l .... r
        Self::query(&self, r) - Self::query(&self, l - 1)
    }
}

#[cfg(test)]
mod test {
    use crate::binary_indexed_tree::BinaryIndexedTree;

    #[test]
    fn t() {
        let data = vec![1, 5, 4, 2, 3];

        let mut my_binary_indexed_tree = BinaryIndexedTree::new(data);

        //         5 5
        // 1 5 4 2 3

        // 1 1 3

        // 2 2 5   should be 14

        // 1 3 -1

        // 1 4 2

        // 2 1 4  should be 16

        for i in 1..=5 {
            BinaryIndexedTree::rep(i);
        }

        my_binary_indexed_tree.add(0, 3);

        println!("{:?} after 1 ", my_binary_indexed_tree.tree);

        assert_eq!(14, my_binary_indexed_tree.query_range(1, 4));

        my_binary_indexed_tree.add(2, -1);

        my_binary_indexed_tree.add(3, 2);

        assert_eq!(16, my_binary_indexed_tree.query_range(0, 3));
    }
}
