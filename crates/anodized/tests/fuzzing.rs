#![no_main]

use anodized::spec;
use arbitrary::{Arbitrary, Unstructured};
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

pub fn __anodized_fn_fuzz_find_insert_index<'__anodized_lifetime, T: Ord>(
    data: &'__anodized_lifetime [u8],
) -> Result<usize, (bool, String)>
where
    Box<[T]>: Arbitrary<'__anodized_lifetime>,
    T: Arbitrary<'__anodized_lifetime>,
{
    let mut unst = Unstructured::new(data);
    let Ok(input_0) = <Box<[T]> as Arbitrary>::arbitrary(&mut unst) else {
        return Err((false, "could not generate input_0".into()));
    };
    let Ok(input_1) = <T as Arbitrary>::arbitrary(&mut unst) else {
        return Err((false, "could not generate input_1".into()));
    };
    __anodized_fn_split_find_insert_index(&input_0, &input_1)
}

fuzz_target!(|data: &[u8]| -> Corpus {
    match __anodized_fn_fuzz_find_insert_index::<i32>(data) {
        Ok(_) => Corpus::Keep,
        Err((false, _)) => Corpus::Reject,
        Err((true, errors)) => panic!("postcondition failed:{errors}"),
    }
});
