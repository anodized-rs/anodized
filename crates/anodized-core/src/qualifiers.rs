use bitflags::bitflags;

bitflags! {
    /// A combination of `fn` qualifiers.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FnQualifiers: u32 {
        /// A deterministic `fn`'s return value only depends on its arguments.
        const DETERMINISTIC = 1 << 0;

        /// An effectless `fn` has no side effects.
        const EFFECTLESS = 1 << 1;

        /// An infallible `fn` does not panic or abort.
        const INFALLIBLE = 1 << 2;

        /// A terminating `fn` does not run forever.
        const TERMINATING = 1 << 3;

        /// A pure `fn` is both deterministic and effectless.
        const PURE = Self::DETERMINISTIC.bits() | Self::EFFECTLESS.bits();

        /// A total `fn` is both infallible and terminating.
        const TOTAL = Self::INFALLIBLE.bits() | Self::TERMINATING.bits();
    }
}
