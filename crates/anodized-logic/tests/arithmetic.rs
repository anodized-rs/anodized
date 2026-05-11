use anodized_logic::arithmetic::int;

#[test]
fn test_basics() {
    assert_eq!(int::from(1) + 1, 2);
    assert_eq!(1 + int::from(1), 2);

    let large_num = int::from(std::u128::MAX) + 42;
    assert_eq!(large_num - 42, std::u128::MAX);
}

#[test]
fn test_collatz() {
    pub fn collatz(mut n: int) -> int {
        while n > 1 {
            n = f(&n);
        }
        n
    }

    fn f(n: &int) -> int {
        if n % 2 == 0 { n / 2 } else { 3 * n + 1 }
    }

    let large_num = int::from(std::u128::MAX) + 42;
    assert_eq!(collatz(large_num), 1);
}
