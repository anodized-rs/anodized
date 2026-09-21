#![no_main]

#[allow(unused_imports)]
use anodized::{spec, unspec};

#[spec(maintains: self.x != "text")]
struct Point {
    #[unspec]
    x: f32,
    #[unspec]
    y: f32,
}
