/// For a type M<A> which can produce types of type M<B> this definition holds:
/// 
/// ```
/// use hkt::HKT;
/// 
/// struct M<T> { t: T };
/// 
/// impl<A, B> HKT<B> for M<A> {
///     type Inner  = A;
///     type Result = M<B>;
/// }
/// ```
pub trait HKT<Output> {
    /// The type contained in the current instance.
    type Inner;
    /// The type containing Output.
    type Result;
}

pub trait Functor<T>: HKT<T> {
    fn map<F>(self, F) -> Self::Result
      where F: Fn(Self::Inner) -> T;
}

pub trait FunctorMut<T>: HKT<T> {
    fn map<F>(self, F) -> Self::Result
      where F: FnMut(Self::Inner) -> T;
}

pub trait FunctorOnce<T>: HKT<T> {
    fn map<F>(self, F) -> Self::Result
      where F: FnOnce(Self::Inner) -> T;
}

pub trait Monad<T>: HKT<T> {
    fn bind<F>(self, F) -> Self::Result
      where F: Fn(Self::Inner) -> Self::Result;
    fn unit(Self::Inner) -> Self
      where Self: HKT<T, Inner=T>;
}

pub trait MonadMut<T>: HKT<T> {
    fn bind<F>(self, F) -> Self::Result
      where F: FnMut(Self::Inner) -> Self::Result;
    fn unit(Self::Inner) -> Self
      where Self: HKT<T, Inner=T>;
}

pub trait MonadOnce<T>: HKT<T> {
    fn bind<F>(self, F) -> Self::Result
      where F: FnOnce(Self::Inner) -> Self::Result;
    fn unit(Self::Inner) -> Self
      where Self: HKT<T, Inner=T>;
}

mod impls {
    use super::{HKT, Functor, FunctorMut, FunctorOnce, Monad, MonadMut, MonadOnce};

    impl<T, M: FunctorMut<T>> Functor<T> for M {
        fn map<F>(self, f: F) -> Self::Result
          where F: FnMut(Self::Inner) -> T {
            M::map(self, f)
        }
    }

    impl<T, M: FunctorOnce<T>> FunctorMut<T> for M {
        fn map<F>(self, f: F) -> Self::Result
          where F: FnOnce(Self::Inner) -> T {
            M::map(self, f)
        }
    }

    impl<T, M: MonadMut<T>> Monad<T> for M {
        fn bind<F>(self, f: F) -> Self::Result
          where F: Fn(Self::Inner) -> Self::Result {
            M::bind(self, f)
        }

        fn unit(t: Self::Inner) -> Self
          where Self: HKT<T, Inner=T> {
            M::unit(t)
        }
    }

    impl<T, M: MonadOnce<T>> MonadMut<T> for M {
        fn bind<F>(self, f: F) -> Self::Result
          where F: FnMut(Self::Inner) -> Self::Result {
            M::bind(self, f)
        }

        fn unit(t: Self::Inner) -> Self
          where Self: HKT<T, Inner=T> {
            M::unit(t)
        }
    }
}

#[test]
fn test_a() {
    #[derive(Debug)]
    struct A<T> {
        t: T,
    }

    impl<T, U> HKT<U> for A<T> {
        type Inner  = T;
        type Result = A<U>;
    }

    impl<T, U> FunctorOnce<U> for A<T> {
        fn map<F>(self, f: F) -> Self::Result
          where F: FnOnce(Self::Inner) -> U {
            A { t: f(self.t) }
        }
    }

    impl<T, U> MonadOnce<U> for A<T> {
        fn bind<F>(self, f: F) -> Self::Result
          where F: FnOnce(Self::Inner) -> Self::Result {
            f(self.t)
        }

        fn unit(t: Self::Inner) -> Self
          where Self: HKT<T, Inner=T> {
            A { t: t }
        }
    }

    let foo = A::<i32>{ t: 2 };
    let bar = A::<i32>{ t: 23 };

    println!("{:?}", Functor::map(foo, |i| format!("{}", i)));
    println!("{:?}", Monad::bind(bar, |i| Monad::unit(format!("{}", i))));
}
