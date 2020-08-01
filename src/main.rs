use recursive::tail_recursive;

fn sum(n: u64) -> u64 {

    #[tail_recursive]
    fn sum_rec(n: u64, a: u64) -> u64 {
        match n {
            0 => return a,
            _ => return sum_rec(n - 1, n + a),
        };
    }

    sum_rec(n, 0)
}

fn main() {
    println!("Result: {}", sum(999_999));
}
