use recursive::recursive;

#[recursive]
fn sum(n: u64, a: u64) -> u64 {
    match n {
        0 => a,
        _ => sum(n - 1, n + a),
    }
}

// enum Action<B, C> {
//     Break(B),
//     Continue(C),
// }

// fn sum(n: u64, a: u64) -> Action<u64, (u64, u64)> {
//     match n {
//         0 => Action::Break(0),
//         _ => Action::Continue((n - 1, n + a)),
//     }
// }

fn main() {
    println!("Result: {}", sum(10, 0));
}
