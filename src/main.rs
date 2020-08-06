use recursive::recursive;

#[recursive]
fn sum(n: u64, a: u64) -> u64 {
    match n {
        0 => a,
        _ => sum(n - 1, n + a),
    }
}

// fn sum(n: u64, a: u64) -> u64 {
//     enum Action<C, R> {
//         Continue(C),
//         Return(R),
//     }
//     fn sum_inner((n, a): (u64, u64)) -> Action<(u64, u64), u64> {
//         Action::Return(42)
//     }
// }

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

// #[recursive]
// fn greet_indefnitely() {
//     let m = 0;
//     if m == 1 {
//         std::process::exit(0);
//     }
//     greet_indefnitely();
// }

fn main() {
    // greet_indefnitely();
    println!("Result: {}", sum(10, 0));
}
