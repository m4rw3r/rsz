/// For a type M<A> which can produce types of type M<B> this definition holds:
/// 
/// ```
/// use rsz::HKT;
/// 
/// struct M<T> { t: T };
/// 
/// impl<A, B> HKT<B> for M<A> {
///     type Inner  = A;
///     type Result = M<B>;
/// }
/// ```
pub trait HKT<Output> {
    type Inner;
    type Result;
}
