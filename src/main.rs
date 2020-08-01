use recursive::recursive;

#[recursive]
fn test(n: u64) -> u64 {
    match n {
        0 => 0,
        _ => n + test(n - 1),
    }
}

fn main() {
    println!("Result: {}", __internal_test_rec(999));
}
