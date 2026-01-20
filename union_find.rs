/// Union-Find for indices 0..N with path compression, union by size, and size tracking.
/// Assumes all elements from 0 to capacity-1 exist (pre-initialized).
pub struct UnionFind {
    pub parent: Vec<usize>,
    pub size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        let size = vec![1; n];
        for (i, p) in parent.iter_mut().enumerate() {
            *p = i;
        }
        Self { parent, size }
    }

    pub fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    /// Returns true if merged
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return false;
        }
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
        true
    }

    pub fn get_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }

    /// Returns HashMap of root -> size for all components (roots only).
    pub fn component_sizes(&mut self) -> std::collections::HashMap<usize, usize> {
        let mut sizes = std::collections::HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            sizes.insert(root, self.size[root]);
        }
        sizes.into_iter().collect()
    }
}

fn main() {
    let mut uf = UnionFind::new(10);
    uf.union(2, 5);
    println!("{}", uf.get_size(2));
}
