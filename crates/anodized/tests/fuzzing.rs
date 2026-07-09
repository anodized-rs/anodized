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

fn __anodized_fn_fuzz_find_insert_index<'__a, T: Ord>(data: &'__a [u8]) -> Corpus
where
    T: Arbitrary<'__a>,
{
    let mut unst = Unstructured::new(data);
    let Ok(input_0) = <Box<[T]> as Arbitrary>::arbitrary(&mut unst) else {
        return Corpus::Reject;
    };
    let Ok(input_1) = <Box<T> as Arbitrary>::arbitrary(&mut unst) else {
        return Corpus::Reject;
    };
    match __anodized_fn_split_find_insert_index(&input_0, &input_1) {
        Ok(_) => Corpus::Keep,
        Err((false, _)) => Corpus::Reject,
        Err((true, errors)) => panic!("postcondition failed:{errors}"),
    }
}

fuzz_target!(|data: &[u8]| -> Corpus { __anodized_fn_fuzz_find_insert_index::<i32>(data) });
