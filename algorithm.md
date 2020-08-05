Given this recursive function:

```rs
fn sum(n: u64) -> u64 {
    match n {
        0 => return 0,
        _ => return n + sum(n - 1),
    }
}
```

Add an accumulator argument (which is the same type as the function return type) and replace all the occurances of the recursive call with that accumulator:

```rs
fn sum(n: u64, a: u64) -> u64 {
    match n {
        0 => return 0,
        _ => return sum(n - 1, n + a),
    }
}
```

```rs

enum Action {
    Break,
    Continue,
}

```
