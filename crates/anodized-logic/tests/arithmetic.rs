use anodized_logic::arithmetic::int;

pub fn collatz(mut n: int) -> int {
    while n > 1 {
        n = f(n);
    }
    n
}

fn f(n: int) -> int {
    if &n % 2 == 0 { n / 2 } else { 3 * n + 1 }
}

struct MyInt(i32);

impl From<MyInt> for int {
    fn from(value: MyInt) -> Self {
        value.0.into()
    }
}

anodized_logic::arithmetic::impl_integral!(MyInt);
