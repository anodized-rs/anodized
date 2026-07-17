#![no_main]

use anodized::spec;
use libfuzzer_sys::{Corpus, fuzz_target};

#[spec(
    requires: seq.is_sorted(),
    ensures: [
        *output <= seq.len(),
        seq[0..*output].iter().all(|item| item < value),
        seq[*output..].iter().all(|item| value <= item),
    ],
)]
pub fn find_insert_index<T: Ord>(seq: &[T], value: &T) -> usize {
    let mut index = 0;
    while index < seq.len() && seq[index] < *value {
        index += 1;
    }
    index
}

#[cfg(feature = "arbitrary")]
pub fn __anodized_fn_arbitrary_find_insert_index<'__anodized_arbitrary, T: Ord, Input0, Input1>(
    data: &'__anodized_arbitrary [u8],
) -> Result<usize, (bool, String)>
where
    Input0: ::std::borrow::Borrow<[T]>,
    Input0: ::anodized::arbitrary::Arbitrary<'__anodized_arbitrary>,
    Input1: ::std::borrow::Borrow<T>,
    Input1: ::anodized::arbitrary::Arbitrary<'__anodized_arbitrary>,
{
    let mut unst = ::anodized::arbitrary::Unstructured::new(data);
    let Ok(input_0) = <Input0 as ::anodized::arbitrary::Arbitrary>::arbitrary(&mut unst) else {
        return Err((false, "could not generate input_0".into()));
    };
    let Ok(input_1) = <Input1 as ::anodized::arbitrary::Arbitrary>::arbitrary(&mut unst) else {
        return Err((false, "could not generate input_1".into()));
    };
    __anodized_fn_split_find_insert_index(input_0.borrow(), input_1.borrow())
}

fuzz_target!(|data: &[u8]| -> Corpus {
    match __anodized_fn_arbitrary_find_insert_index::<i32, Box<[i32]>, i32>(data) {
        Ok(_) => Corpus::Keep,
        Err((false, _)) => Corpus::Reject,
        Err((true, errors)) => panic!("postcondition failed:{errors}"),
    }
});
