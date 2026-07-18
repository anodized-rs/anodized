#![no_main]

use libfuzzer_sys::arbitrary::{Arbitrary, Error, Unstructured};
use libfuzzer_sys::{fuzz_target, Corpus};

fuzz_target!(|data: &[u8]| -> Corpus {
    let mut unst = Unstructured::new(data);
    let Ok((seq, value)) = <(AscendingVec<i32>, i32) as Arbitrary>::arbitrary(&mut unst) else {
        return Corpus::Reject;
    };
    match ::bin_search::__anodized_fn_split_bin_search(&seq, &value) {
        Ok(_) => Corpus::Keep,
        Err((false, _)) => Corpus::Reject,
        Err((true, errors)) => {
            eprintln!("inputs:");
            dbg!(seq);
            dbg!(value);
            panic!("postcondition failed:{errors}");
        }
    }
});

/// Helper newtype to more efficiently generate valid inputs.
#[derive(Debug, Clone)]
struct AscendingVec<T>(Vec<T>);

impl<'a, T: Ord + Arbitrary<'a>> Arbitrary<'a> for AscendingVec<T> {
    fn arbitrary(unst: &mut Unstructured<'a>) -> Result<Self, Error> {
        let mut raw_seq = <Vec<T> as Arbitrary>::arbitrary(unst)?;
        raw_seq.sort();
        Ok(Self(raw_seq))
    }
}

impl<T> std::ops::Deref for AscendingVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
