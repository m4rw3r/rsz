use ::hkt::HKT;

// Unit needs a separate trait since Monad and Applicative require Fn generics which cannot
// be inferred when calling unit
pub trait Unit<T>: HKT<T> {
    fn unit(Self::Inner) -> Self
      where Self: HKT<T, Inner=T>;
}

/// The Monad trait, specifying ``bind``.
// TODO: How to enforce that F: Fn*(Self::Inner) -> Self::Result?
//       Seems to require impl-specialization
pub trait Monad<T, F>: Unit<T> {
    fn bind(self, F) -> Self::Result;
}
