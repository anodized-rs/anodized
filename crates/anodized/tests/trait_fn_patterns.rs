#![allow(unused)]

use anodized::spec;

#[spec]
trait Trait {
    #[spec(requires: left < right)]
    fn func_1(input @ (left, right): (i32, i32)) {
        unimplemented!()
    }

    #[spec(requires: lower < upper)]
    fn func_2(Bounds { lower, upper }: Bounds) {
        unimplemented!()
    }

    #[spec(
        requires: [
            lower < upper,
            i >= i,
            i < upper,
        ],
    )]
    fn func_3((i, Bounds { lower, upper }): (i32, Bounds)) {
        unimplemented!()
    }
}

struct Bounds {
    lower: i32,
    upper: i32,
}

struct Type;

#[spec]
impl Trait for Type {
    fn func_1(_: (i32, i32)) {}
    fn func_2(_: Bounds) {}
    fn func_3(_: (i32, Bounds)) {}
}

#[cfg(all(anodized_print, anodized_panic))]
#[test]
#[should_panic(expected = r#"precondition failed:
    left < right"#)]
fn test_1() {
    let _ = Type::func_1((42, 1));
}

#[cfg(all(anodized_print, anodized_panic))]
#[test]
#[should_panic(expected = r#"precondition failed:
    lower < upper"#)]
fn test_2() {
    let _ = Type::func_2(Bounds {
        lower: 42,
        upper: 1,
    });
}

#[cfg(all(anodized_print, anodized_panic))]
#[test]
#[should_panic(expected = r#"precondition failed:
    i < upper"#)]
fn test_3() {
    let _ = Type::func_3((42, Bounds { lower: 1, upper: 5 }));
}
