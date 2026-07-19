/// Try to call a function with a [crate::spec] and return a [Result].
///
/// The macro must wrap a call expression of the following supported cases:
/// ```
/// // free function with a qualified name
/// let result = try_call! { qualified::free_fn(...) };
/// ```
///
/// ```
/// // method
/// let result = try_call! { receiver.method(...) };
/// ```
///
/// ```
/// // function qualified by a type or trait
/// let result = try_call! { Type::associated_fn(...) };
/// let result = try_call! { <Type>::associated_fn(...) };
/// let result = try_call! { <Type as Trait>::trait_fn(...) };
/// ```
pub use anodized_macros::try_call;

/// Return type of a call wrapped by [try_call].
pub type Result<T> = std::result::Result<T, Error<T>>;

/// Error that represents a pre/postcondition failure.
pub enum Error<T> {
    /// Preconditions failed.
    Pre(Messages),
    /// Postconditions failed.
    Post(T, Messages),
}

pub type Messages = String;
