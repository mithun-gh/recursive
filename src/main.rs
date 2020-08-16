use recursive::recursive;

#[recursive]
fn sum(n: u64, a: u64) -> u64 {
    match n {
        0 => return a,
        _ => return sum(n - 1, n + a),
    }
}

// #[recursive]
// fn factorial(n: u64, a: u64) -> u64 {
//     if n == 0 {
//         a
//     } else {
//         factorial(n - 1, n * a)
//     }
// }

// #[recursive]
// fn repeat(input: &str, n: usize, a: String) -> String {
//     if n == 0 {
//         a
//     } else {
//         repeat(input, n - 1, format!("{}{}", input, a))
//     }
// }

// struct Arith(u64);

// impl Arith {
//     #[recursive]
//     fn sum(&self, n: u64, a: u64) -> u64 {
//         match n {
//             0 => a,
//             _ => self.sum(n - 1, n + a),
//         }
//     }
// }

fn main() {
    println!("Result: {}", sum(999_999, 0));
    // println!("Result: {}", factorial(10, 1));
    // println!("{}", repeat("*", 10, String::new()));

    // let arith = Arith(12);
    // println!("Result: {}", arith.sum(10, 0));
}
