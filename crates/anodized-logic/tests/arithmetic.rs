use anodized_logic::arithmetic::int;

pub fn collatz(mut n: int) -> int {
    while n > 1 {
        n = f(&n);
    }
    n
}

fn f(n: &int) -> int {
    if n % 2 == 0 { n / 2 } else { 3 * n + 1 }
}

#[test]
fn int_basics() {
    assert_eq!(int!(1) + 1, 2);
    assert_eq!(1 + int!(1), 2);

    let large_num = int!(340_282_366_920_938_463_463_374_607_431_768_211_456);
    assert_eq!(
        large_num - 1,
        int!(340_282_366_920_938_463_463_374_607_431_768_211_455),
    );
}
