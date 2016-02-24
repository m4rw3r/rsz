#![feature(specialization)]
mod impls;


/*
struct Data;

trait Foo<T> {}
trait Bar<T> {}

impl<T, U> Bar<U> for T    where T: Foo<U> {}
// impl<U>    Foo<U> for Data                 {}
impl<U>    Bar<U> for Data                 {}
*/

/*
pub struct Data;

trait Foo {}
trait Bar {}

impl<T> Bar for T    where T: Foo {}
impl    Bar for Data              {}
*/

/// Trait for a type which contains another type
// Unit needs a separate trait since Monad and Applicative require Fn generics which cannot
// be inferred when calling unit
pub trait Unit {
    type Inner;

    fn unit(Self::Inner) -> Self;
}

/// For a type M<A> which can produce types of type M<B> this definition holds:
///
/// ```
/// use rsz::{HKT, Unit};
///
/// struct M<T> { t: T };
///
/// impl<A> Unit for M<A> {
///     type Inner = A;
///
///     fn unit(a: A) -> Self { M { t: a } }
/// }
///
/// impl<A, B> HKT<B> for M<A> {
///     type Result = M<B>;
/// }
/// ```
pub trait HKT<Output>: Unit {
    type Result;
}

pub trait FunctorOnce<T>: HKT<T> {
    fn map<F>(self, F) -> Self::Result
      where F: FnOnce(Self::Inner) -> T;
}

pub trait FunctorMut<T>: HKT<T> {
    fn map<F>(self, F) -> Self::Result
      where F: FnMut(Self::Inner) -> T;
}

pub trait Functor<T>: HKT<T> {
    fn map<F>(self, F) -> Self::Result
      where F: Fn(Self::Inner) -> T;
}

/// The Monad trait, specifying ``bind``.
pub trait MonadOnce<T>: HKT<T> {
    fn bind<F>(self, f: F) -> Self::Result
      where F: FnOnce(Self::Inner) -> Self::Result;
}

pub trait MonadMut<T>: HKT<T> {
    fn bind<F>(self, f: F) -> Self::Result
      where F: FnMut(Self::Inner) -> Self::Result;
}

pub trait Monad<T>: HKT<T> {
    fn bind<F>(self, f: F) -> Self::Result
      where F: Fn(Self::Inner) -> Self::Result;
}

impl<T, U> FunctorMut<U> for T
  where T: FunctorOnce<U> {
    fn map<F>(self, f: F) -> Self::Result
      where F: FnMut(Self::Inner) -> U {
        FunctorOnce::map(self, f)
    }
}

impl<T, U> Functor<U> for T
  where T: FunctorMut<U> {
    fn map<F>(self, f: F) -> Self::Result
      where F: Fn(Self::Inner) -> U {
        FunctorMut::map(self, f)
    }
}

impl<T, U> MonadMut<U> for T
  where T: MonadOnce<U> {
    fn bind<F>(self, f: F) -> Self::Result
      where F: FnMut(Self::Inner) -> Self::Result {
        MonadOnce::bind(self, f)
    }
}

impl<T, U> Monad<U> for T
  where T: MonadMut<U> {
    fn bind<F>(self, f: F) -> Self::Result
      where F: Fn(Self::Inner) -> Self::Result {
        MonadMut::bind(self, f)
    }
}

#[cfg(test)]
mod test {
    use ::{HKT, Functor, FunctorOnce, Unit, Monad, MonadOnce};

    #[test]
    fn test_simple_container() {
        #[derive(Debug)]
        struct A<T> {
            t: T,
        }

        impl<T> Unit for A<T> {
            type Inner = T;

            fn unit(t: T) -> Self { A { t: t } }
        }

        impl<T, U> HKT<U> for A<T> {
            type Result = A<U>;
        }

        impl<T, U> FunctorOnce<U> for A<T> {
            fn map<F>(self, f: F) -> Self::Result
              where F: FnOnce(T) -> U {
                A { t: f(self.t) }
            }
        }

        impl<T, U> MonadOnce<U> for A<T> {
            fn bind<F>(self, f: F) -> Self::Result
              where F: FnOnce(T) -> Self::Result {
                f(self.t)
            }
        }

        let foo = A::<i32>{ t: 2 };
        let bar = A::<i32>{ t: 23 };

        println!("{:?}", Functor::map(foo, |i| format!("{}", i)));
        println!("{:?}", Monad::bind(bar, |i| Unit::unit(format!("{}", i))));
    }
}
