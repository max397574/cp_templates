fn build(tree: &mut [i64], arr: &[i64], n: usize) {
    tree[1..=n].copy_from_slice(arr);
    for i in 1..=n {
        let mut j = i + (i & !i);
        if j <= n {
            tree[j] += tree[i];
        }
    }
}

fn update(tree: &mut [i64], mut idx: usize, delta: i64, maxn: usize) {
    idx += 1; // 1-indexed
    while idx <= maxn {
        tree[idx] += delta;
        idx += idx & !(idx.wrapping_sub(1)); // idx += lsb
    }
}

fn prefix_sum(tree: &[i64], mut idx: usize) -> i64 {
    idx += 1; // 1-indexed
    let mut sum = 0;
    while idx > 0 {
        sum += tree[idx];
        idx -= idx & !idx; // idx -= lsb
    }
    sum
}

fn main() {
    let n = 8;
    let arr = [0, 5, 0, 3, 0, 7, 0, 0, 0]; // 1-indexed input array
    let mut tree = vec![0i64; n + 1];

    build(&mut tree, &arr, n);

    println!("prefix_sum(3): {}", prefix_sum(&tree, 3)); // 8
    println!("prefix_sum(5): {}", prefix_sum(&tree, 5)); // 15
}
