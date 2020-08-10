use recursive::recursive;

#[recursive]
fn sum(n: u64, a: u64) -> u64 {
    match n {
        0 => a,
        _ => sum(n - 1, n + a),
    }
}

#[recursive]
fn factorial(n: u64, a: u64) -> u64 {
    if n == 0 {
        a
    } else {
        factorial(n - 1, n * a)
    }
}

#[recursive]
fn repeat(input: &str, n: usize, a: String) -> String {
    if n == 0 {
        a
    } else {
        repeat(input, n - 1, format!("{}{}", input, a))
    }
}

fn main() {
    println!("Result: {}", sum(999_999, 0));
    println!("Result: {}", factorial(10, 1));
    println!("{}", repeat("*", 10, String::new()));
}
