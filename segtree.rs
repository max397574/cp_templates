fn build(arr: &[i64]) -> Vec<i64> {
    let n = arr.len();
    let mut tree = vec![0; 2 * n];
    for i in 0..n {
        tree[n + i] = arr[i];
    }
    for i in (1..n).rev() {
        tree[i] = tree[2 * i] + tree[2 * i + 1];
    }
    tree
}

// Update value at index idx
fn update(tree: &mut [i64], n: usize, mut idx: usize, val: i64) {
    idx += n;
    tree[idx] = val;

    while idx > 1 {
        idx /= 2;
        tree[idx] = tree[2 * idx] + tree[2 * idx + 1];
    }
}

// Query range [l, r)
fn query(tree: &[i64], n: usize, mut l: usize, mut r: usize) -> i64 {
    l += n;
    r += n;
    let mut sum = 0;

    while l < r {
        if l % 2 == 1 {
            sum += tree[l];
            l += 1;
        }
        if r % 2 == 1 {
            r -= 1;
            sum += tree[r];
        }
        l /= 2;
        r /= 2;
    }

    sum
}

fn main() {
    let arr = vec![1, 3, 5, 7, 9, 11];
    let n = arr.len();
    let mut tree = build(&arr);

    println!("Sum [0, 6): {}", query(&tree, n, 0, 6)); // 36
    println!("Sum [1, 4): {}", query(&tree, n, 1, 4)); // 15

    update(&mut tree, n, 2, 10);
    println!("After update:");
    println!("Sum [0, 6): {}", query(&tree, n, 0, 6)); // 41
}
