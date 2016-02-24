use ::{FunctorOnce, HKT, MonadOnce, Unit};

impl<T, E> Unit for Result<T, E> {
    type Inner = T;

    fn unit(t: Self::Inner) -> Self { Ok(t) }
}

impl<T, U, E> HKT<U> for Result<T, E> {
    type Result = Result<U, E>;
}

impl<T, U, E> FunctorOnce<U> for Result<T, E> {
    fn map<F>(self, f: F) -> Self::Result
      where F: FnOnce(T) -> U {
        self.map(f)
    }
}

impl<T, U, E> MonadOnce<U> for Result<T, E> {
    fn bind<F>(self, f: F) -> Self::Result
      where F: FnOnce(T) -> Result<U, E> {
        match self {
            Ok(t)  => f(t),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use ::{Monad, Unit};

    #[test]
    fn left_identity() {
        fn f(i: u32) -> Result<i32, ()> {
            Unit::unit((i as i32) - 1)
        }

        // return a >>= f
        let lhs = Monad::bind(Result::unit(1), f);
        // f a
        let rhs = f(1);

        assert_eq!(lhs, rhs);
    }

    #[test]
    fn right_identity() {
        // m >>= return
        let lhs: Result<_, ()> = Monad::bind(Ok(1), Unit::unit);
        // m
        let rhs = Ok(1);

        assert_eq!(lhs, rhs);
    }

    #[test]
    fn associativity() {
         fn f(num: u32) -> Result<u64, ()> {
            Unit::unit((num + 1) as u64)
        }

        fn g(num: u64) -> Result<u64, ()> {
            Unit::unit(num * 2)
        }

        // (m >>= f) >>= g
        let lhs = Monad::bind(Monad::bind(Ok(2), f), g);
        // m >>= (\x -> f x >> g)
        let rhs = Monad::bind(Ok(2), |x| Monad::bind(f(x), g));

        assert_eq!(lhs, rhs);
    }
}

#[cfg(test)]
mod test_mut {
    use {Unit, FunctorMut, MonadMut};

    #[test]
    fn map() {
        let mut run_a = false;
        let mut run_b = false;

        let a: Result<_, ()> = Ok(1i32);
        let b: Result<i32, _> = Err(());

        let r_a = a.map(|i| {
            run_a = true;

            i + 1
        });
        let r_b = b.map(|i| {
            run_b = true;

            i + 2
        });

        assert_eq!(r_a, Ok(2));
        assert_eq!(run_a, true);
        assert_eq!(r_b, Err(()));
        assert_eq!(run_b, false);
    }

    #[test]
    fn map_trait() {
        let mut run_a = false;
        let mut run_b = false;

        let a: Result<_, ()> = Ok(1i32);
        let b: Result<i32, _> = Err(());

        let r_a = FunctorMut::map(a, |i| {
            run_a = true;

            i + 1
        });
        let r_b = FunctorMut::map(b, |i| {
            run_b = true;

            i + 2
        });

        assert_eq!(r_a, Ok(2));
        assert_eq!(run_a, true);
        assert_eq!(r_b, Err(()));
        assert_eq!(run_b, false);
    }

    #[test]
    fn unit() {
        let r: Result<_, ()> = Unit::unit(123i32);

        assert_eq!(r, Ok(123i32));
    }

    #[test]
    fn bind() {
        let mut run_a = false;
        let mut run_b = false;

        // Here the type-annotation is actually legitimately needed
        let a: Result<_, ()>  = Unit::unit(1i32);
        let b: Result<i32, _> = Err(());

        let r_a = a.bind(|j| {
            run_a = true;

            Unit::unit(j + 1)
        });
        let r_b = b.bind(|j| {
            run_b = true;

            Unit::unit(j + 2)
        });

        assert_eq!(r_a, Ok(2));
        assert_eq!(run_a, true);
        assert_eq!(r_b, Err(()));
        assert_eq!(run_b, false);
    }

    #[test]
    fn bind_trait() {
        let mut run_a = false;
        let mut run_b = false;

        // Here the type-annotation is actually legitimately needed
        let a: Result<_, ()>  = Unit::unit(1i32);
        let b: Result<i32, _> = Err(());

        let r_a = MonadMut::bind(a, |j| {
            run_a = true;

            Unit::unit(j + 1)
        });
        let r_b = MonadMut::bind(b, |j| {
            run_b = true;

            Unit::unit(j + 2)
        });

        assert_eq!(r_a, Ok(2));
        assert_eq!(run_a, true);
        assert_eq!(r_b, Err(()));
        assert_eq!(run_b, false);
    }
}
