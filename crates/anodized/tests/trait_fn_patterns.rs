use anodized::spec;

#[spec]
trait PatternArguments {
    #[spec(requires: left <= right)]
    fn func((left, right): (i32, i32)) {
        let _ = (left, right);
    }
}

struct Bounds {
    lower: i32,
    upper: i32,
}

#[spec]
trait PatternArguments {
    #[spec(requires: lower <= upper)]
    fn func(Bounds { lower, upper }: Bounds) {
        let _ = (lower, upper);
    }
}
