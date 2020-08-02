use recursive::recursive;

#[allow(unused)]
#[recursive]
fn sum(n: u64, a: u64) -> u64 {
    match n {
        0 => return a,
        _ => return sum(n - 1, n + a),
    }
}

/*

fn sum(n: u64, a: u64) -> u64 {
    match n {
        0 => return a,
        _ => return sum(n - 1, n + a),
    }
}

*/

fn main() {
    // println!("Result: {}", sum(10, 0));
}
