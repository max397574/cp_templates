pub fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    let mut c;
    while b != 0 {
        c = a;
        a = b;
        b = c % b;
    }
    a
}

pub fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}
