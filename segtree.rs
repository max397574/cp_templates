// Build segment tree from array
// tree size is 2*n, data goes in indices [n..2n)
fn build<T: Clone>(arr: &[T], op: impl Fn(&T, &T) -> T) -> Vec<T> {
    let n = arr.len();
    let mut tree = Vec::with_capacity(2 * n);

    for _ in 0..n {
        tree.push(arr[0].clone());
    }
    for val in arr.iter().take(n) {
        tree.push(val.clone());
    }
    for i in (1..n).rev() {
        tree[i] = op(&tree[2 * i], &tree[2 * i + 1]);
    }

    tree
}

// Update value at index idx
fn update<T: Clone>(tree: &mut [T], n: usize, mut idx: usize, val: T, op: impl Fn(&T, &T) -> T) {
    idx += n;
    tree[idx] = val;

    while idx > 1 {
        idx /= 2;
        tree[idx] = op(&tree[2 * idx], &tree[2 * idx + 1]);
    }
}

// Query range [l, r)
fn query<T: Clone>(
    tree: &[T],
    n: usize,
    mut l: usize,
    mut r: usize,
    identity: T,
    op: impl Fn(&T, &T) -> T,
) -> T {
    l += n;
    r += n;
    let mut left_res = identity.clone();
    let mut right_res = identity;

    while l < r {
        if l % 2 == 1 {
            left_res = op(&left_res, &tree[l]);
            l += 1;
        }
        if r % 2 == 1 {
            r -= 1;
            right_res = op(&tree[r], &right_res);
        }
        l /= 2;
        r /= 2;
    }

    op(&left_res, &right_res)
}

fn main() {
    // Example 1: Sum
    let arr = vec![1, 3, 5, 7, 9, 11];
    let n = arr.len();
    let mut tree = build(&arr, |a, b| a + b);

    println!("Sum [0, 6): {}", query(&tree, n, 0, 6, 0, |a, b| a + b));
    println!("Sum [1, 4): {}", query(&tree, n, 1, 4, 0, |a, b| a + b));

    update(&mut tree, n, 2, 10, |a, b| a + b);
    println!("After update:");
    println!("Sum [0, 6): {}", query(&tree, n, 0, 6, 0, |a, b| a + b));

    // Example 2: Min
    let arr2 = vec![5, 2, 8, 1, 9, 3];
    let n2 = arr2.len();
    let tree2 = build(&arr2, |a, b| *a.min(b));

    println!(
        "\nMin [0, 6): {}",
        query(&tree2, n2, 0, 6, i32::MAX, |a, b| *a.min(b))
    );
    println!(
        "Min [1, 4): {}",
        query(&tree2, n2, 1, 4, i32::MAX, |a, b| *a.min(b))
    );

    // Example 3: Max
    let tree3 = build(&arr2, |a, b| *a.max(b));
    println!(
        "\nMax [0, 6): {}",
        query(&tree3, n2, 0, 6, i32::MIN, |a, b| *a.max(b))
    );
}
