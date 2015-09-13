mod hkt;
mod monad;
mod functor;
mod impls;

pub use hkt::HKT;
pub use monad::{Monad, Unit};
pub use functor::Functor;

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

        impl<T> Unit for A<T> {
            type Inner = T;

            fn unit(t: T) -> Self {
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
