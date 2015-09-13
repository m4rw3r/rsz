use ::hkt::HKT;

// TODO: How to enforce that F: Fn*(Self::Inner) -> T?
//       Seems to require impl-specialization
pub trait Functor<T, F>: HKT<T> {
    fn map(self, F) -> Self::Result;
}
