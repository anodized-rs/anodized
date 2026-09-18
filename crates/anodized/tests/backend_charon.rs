#![cfg_attr(anodized_charon, feature(register_tool))]
#![cfg_attr(anodized_charon, register_tool(charon))]

use anodized::spec;

#[spec]
pub fn f() {
    todo!()
}

#[spec]
trait T {
    #[spec]
    fn f();

    #[spec]
    fn g() {
        todo!()
    }
}

pub struct S;

#[spec]
impl T for S {
    #[spec]
    fn f() {
        todo!()
    }

    #[spec]
    fn g() {
        todo!()
    }
}
