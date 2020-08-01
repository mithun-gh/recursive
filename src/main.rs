use recursive::tail_recursive;

fn _sum(n: u64) -> u64 {

    #[tail_recursive]
    fn sum_rec(n: u64, a: u64) -> u64 {
        match n {
            0 => return a,
            _ => return sum_rec(n - 1, n + a),
        };
    }

    sum_rec(n, 0)
}

enum Action<C, R> {
    Continue(C),
    Return(R),
}

fn sum_tc((n, a): (u64, u64)) -> Action<(u64, u64), u64> {
    match n {
        0 => return Action::Return(a),
        _ => return Action::Continue((n - 1, n + a)),
    };
}

fn executor(n: u64) -> u64 {
    let mut acc = (n, 0);
    loop {
        match sum_tc(acc) {
            Action::Return(_) => break,
            Action::Continue(v) => acc = v,
        }
    }
    acc.1
}

fn main() {
    // println!("Result: {}", sum(999_999));
    println!("Result: {}", executor(999_999));
}
