#![no_main]

use anodized::spec;

#[spec(maintains: self.x != "text")]
struct Point {
    x: f32,
    y: f32,
}
