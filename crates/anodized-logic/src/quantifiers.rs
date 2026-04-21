#[allow(private_bounds)]
pub fn forall<T, P: Predicate<T>>(_predicate: P) -> bool {
    unimplemented!("Runtime checks are not supported for quantifiers.")
}

#[allow(private_bounds)]
pub fn exists<T, P: Predicate<T>>(_predicate: P) -> bool {
    unimplemented!("Runtime checks are not supported for quantifiers.")
}

trait Predicate<T> {}

impl<P, T1> Predicate<(T1,)> for P where P: Fn(T1) -> bool {}
impl<P, T1, T2> Predicate<(T1, T2)> for P where P: Fn(T1, T2) -> bool {}
impl<P, T1, T2, T3> Predicate<(T1, T2, T3)> for P where P: Fn(T1, T2, T3) -> bool {}
impl<P, T1, T2, T3, T4> Predicate<(T1, T2, T3, T4)> for P where P: Fn(T1, T2, T3, T4) -> bool {}
impl<P, T1, T2, T3, T4, T5> Predicate<(T1, T2, T3, T4, T5)> for P where
    P: Fn(T1, T2, T3, T4, T5) -> bool
{
}
