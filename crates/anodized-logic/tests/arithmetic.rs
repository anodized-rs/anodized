use anodized_logic::arithmetic::int;

fn f(n: int) -> int {
    if n % 2 == 0 { n / 2 } else { 3 * n + 1 }
}

fn collatz(mut n: int) -> int {
    while n > 1 {
        n = f(n);
    }
    n
}
