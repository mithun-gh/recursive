use recursive::recursive;

#[recursive]
fn sum(n: u64, a: u64) -> u64 {
    match n {
        0 => return a,
        _ => return sum(n - 1, n + a),
    }
}

#[recursive]
fn factorial(n: u64, a: u64) -> u64 {
    if n == 0 {
        return a;
    } else {
        return factorial(n - 1, n * a);
    }
}

fn main() {
    println!("Result: {}", sum(999_999, 0));
    println!("Result: {}", factorial(10, 1));
}
