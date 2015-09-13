use ::hkt::HKT;

// Unit needs a separate trait since Monad and Applicative require Fn generics which cannot
// be inferred when calling unit
pub trait Unit {
    type Inner;

    fn unit(Self::Inner) -> Self;
}

/// The Monad trait, specifying ``bind``.
// TODO: How to enforce that F: Fn*(Self::Inner) -> Self::Result?
//       Seems to require impl-specialization
// TODO: How to enforce that HKT::Inner == Unit::Inner?
pub trait Monad<T, F>: HKT<T> + Unit {
    fn bind(self, F) -> Self::Result;
}
