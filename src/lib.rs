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
    type Inner;
    type Result;
}

// TODO: How to enforce that F: Fn*(Self::Inner) -> T?
//       Seems to require impl-specialization
pub trait Functor<T, F>: HKT<T> {
    fn map(self, F) -> Self::Result;
}

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

/*
pub fn map<T, M: Functor<T, F>, F>(m: M, f: F) -> M::Result {
    M::map(m, f)
}

pub fn unit<T, M>(t: M::Inner) -> M
  where M: Unit<T, Inner=T> {
    Unit::unit(t)
}

pub fn bind<M: Monad<T, F>, T, F>(m: M, f: F) -> M::Result {
    Monad::bind(m, f)
}
*/

#[cfg(test)]
mod test {
    use ::{HKT, Functor, Unit, Monad};

    #[test]
    fn test_simple_container() {
        #[derive(Debug)]
        struct A<T> {
            t: T,
        }

        impl<T, U> HKT<U> for A<T> {
            type Inner  = T;
            type Result = A<U>;
        }

        impl<T, U, F> Functor<U, F> for A<T>
          where F: FnOnce(T) -> U {
            fn map(self, f: F) -> Self::Result {
                A { t: f(self.t) }
            }
        }

        impl<T, U> Unit<U> for A<T> {
            fn unit(t: Self::Inner) -> Self
              where Self: HKT<T, Inner=T> {
                A { t: t }
            }
        }

        impl<T, U, F> Monad<U, F> for A<T>
          where F: FnOnce(T) -> A<U> {
            fn bind(self, f: F) -> Self::Result {
                f(self.t)
            }
        }

        let foo = A::<i32>{ t: 2 };
        let bar = A::<i32>{ t: 23 };

        println!("{:?}", Functor::map(foo, |i| format!("{}", i)));
        println!("{:?}", Monad::bind(bar, |i| Unit::unit(format!("{}", i))));
    }
}
