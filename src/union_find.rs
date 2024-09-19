pub struct UnionFind {
    // data: Vec<usize>,
    union: Vec<usize>,
    rank: Vec<usize>,
    counter: usize, // 集合总数
}

impl UnionFind {
    pub fn new(data_size: usize) -> Self {
        let rank = vec![1; data_size];

        let mut union = vec![0; data_size];
        for i in 0..data_size {
            union[i] = i;
        }

        UnionFind {
            counter: union.len(),
            union,
            rank,
        }
    }

    pub fn get_counter(&self) -> usize {
        self.counter
    }

    pub fn find(&mut self, a: usize) -> usize {
        let mut parent = self.union[a];

        // while parent != self.union[parent] {
        //     parent = self.union[parent];
        // }

        // 路径压缩:
        while parent != self.union[parent] {
            self.union[parent] = self.union[self.union[parent]];
            parent = self.union[parent];
        }

        parent
    }

    pub fn is_connected(&mut self, a: usize, b: usize) -> bool {
        Self::find(self, a) == Self::find(self, b)
    }

    // 合并
    pub fn union(&mut self, a: usize, b: usize) {
        let a_root = Self::find(self, a);
        let b_root = Self::find(self, b);

        if a_root == b_root {
            // println!("{:?} {:?}    {:?} {:?}", a, a_root, b, b_root);
            return;
        }

        if self.rank[a_root] > self.rank[b_root] {
            self.union[b_root] = a_root;
        } else if self.rank[a_root] < self.rank[b_root] {
            self.union[a_root] = b_root;
        } else {
            // 随便选一个
            self.union[a_root] = b_root;
            self.rank[b_root] += 1;
        }
        self.counter -= 1;
    }

    // 孤立
    pub fn isolate(&mut self, a: usize) {
        self.union[a] = a;
    }

    pub fn is_alone(&mut self, a: usize) -> bool {
        if self.find(a) != a {
            return false; // 父亲不是自己, 说明自己不是单独的集合.
        }

        // 寻找是不是还有节点的父亲是自己
        for idx in 0..self.union.len() {
            if idx != a && self.find(idx) == a {
                return false;
            }
        }

        true
    }

    // 每个集合有多少元素.
    pub fn get_group_counter(&mut self) -> Vec<usize> {
        // 父亲的孩子个数包括自己:  0 1 2 3
        //                     [3 3 2 2]

        let mut res = vec![0; self.union.len()];
        for i in 0..self.union.len() {
            let father_idx = Self::find(self, i);
            res[father_idx] += 1;
        }

        res
    }
}

#[cfg(test)]
mod tests {

    use super::UnionFind;

    #[test]
    fn it_works() {
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut my_union = UnionFind::new(data.len());

        my_union.union(0, 1);
        my_union.union(1, 2);
        my_union.union(1, 3);
        // my_union.union(0, 1);
        for i in 0..10 {
            println!("find {} belongs to  {:?}", i, my_union.find(i));
        }
    }
}
