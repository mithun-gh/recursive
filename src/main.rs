use recursive::recursive;

#[recursive]
fn sum(n: i32) -> i32 {
    match n {
        0 => 0,
        _ => n + sum(n - 1),
    }
}

fn main() {
    println!("Sum: {}", sum(10));
}
