#![no_main]

use libfuzzer_sys::arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::{fuzz_target, Corpus};

fuzz_target!(|data: &[u8]| -> Corpus {
    let mut unst = Unstructured::new(data);
    let Ok((in0, in1)) = <(Vec<i32>, i32) as Arbitrary>::arbitrary(&mut unst) else {
        return Corpus::Reject;
    };
    match ::bin_search::__anodized_fn_split_bin_search(&in0, &in1) {
        Ok(_) => Corpus::Keep,
        Err((false, _)) => Corpus::Reject,
        Err((true, errors)) => panic!("postcondition failed:{errors}"),
    }
});
