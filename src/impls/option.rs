use ::{FunctorOnce, HKT, MonadOnce, Unit};

impl<T> Unit for Option<T> {
    type Inner = T;

    fn unit(t: Self::Inner) -> Self { Some(t) }
}

impl<T, U> HKT<U> for Option<T> {
    type Result = Option<U>;
}

impl<T, U> FunctorOnce<U> for Option<T> {
    fn map<F>(self, f: F) -> Self::Result
      where F: FnOnce(T) -> U {
        self.map(f)
    }
}

impl<T, U> MonadOnce<U> for Option<T> {
    fn bind<F>(self, f: F) -> Self::Result
      where F: FnOnce(T) -> Option<U> {
        match self {
            Some(t) => f(t),
            None    => None,
        }
    }
}

#[cfg(test)]
mod test {
    use ::{FunctorOnce, Monad, Unit};

    #[test]
    fn left_identity() {
        fn f(i: u32) -> Option<i32> {
            Unit::unit((i as i32) - 1)
        }

        // return a >>= f
        let lhs = Monad::bind(Option::unit(1), f);
        // f a
        let rhs = f(1);

        assert_eq!(lhs, rhs);
    }

    #[test]
    fn right_identity() {
        // m >>= return
        let lhs = Monad::bind(Some(1), Unit::unit);
        // m
        let rhs = Some(1);

        assert_eq!(lhs, rhs);
    }

    #[test]
    fn associativity() {
         fn f(num: u32) -> Option<u64> {
            Unit::unit((num + 1) as u64)
        }

        fn g(num: u64) -> Option<u64> {
            Unit::unit(num * 2)
        }

        // (m >>= f) >>= g
        let lhs = Monad::bind(Monad::bind(Some(2), f), g);
        // m >>= (\x -> f x >> g)
        let rhs = Monad::bind(Some(2), |x| Monad::bind(f(x), g));

        assert_eq!(lhs, rhs);
    }

    #[test]
    fn map() {
        let mut run_a = false;
        let mut run_b = false;

        let a = Some(1i32);
        let b: Option<i32> = None;

        let r_a = a.map(|i| {
            run_a = true;

            i + 1
        });
        let r_b = b.map(|i| {
            run_b = true;

            i + 2
        });

        assert_eq!(r_a, Some(2));
        assert_eq!(run_a, true);
        assert_eq!(r_b, None);
        assert_eq!(run_b, false);
    }

    #[test]
    fn map_trait() {
        let mut run_a = false;
        let mut run_b = false;

        let a = Some(1i32);
        let b: Option<i32> = None;

        let r_a = FunctorOnce::map(a, |i| {
            run_a = true;

            i + 1
        });
        let r_b = FunctorOnce::map(b, |i| {
            run_b = true;

            i + 2
        });

        assert_eq!(r_a, Some(2));
        assert_eq!(run_a, true);
        assert_eq!(r_b, None);
        assert_eq!(run_b, false);
    }

    #[test]
    fn unit() {
        // TODO: Why is the annotation required here? If we look at the output generated from
        // let r: () = Unit::unit(123i32); then it tells us that it is an Option<i32>???
        let r: Option<_> = Unit::unit(123i32);

        assert_eq!(r, Some(123i32));
    }
}

#[cfg(test)]
mod test_mut {
    use {Unit, MonadMut};

    #[test]
    fn bind() {
        let mut run_a = false;
        let mut run_b = false;

        // Here the type-annotation is actually legitimately needed
        let a: Option<_>   = Unit::unit(1i32);
        let b: Option<i32> = None;

        let r_a = a.bind(|j| {
            run_a = true;

            Unit::unit(j + 1)
        });
        let r_b = b.bind(|j| {
            run_b = true;

            Unit::unit(j + 2)
        });

        assert_eq!(r_a, Some(2));
        assert_eq!(run_a, true);
        assert_eq!(r_b, None);
        assert_eq!(run_b, false);
    }

    #[test]
    fn bind_trait() {
        use MonadMut;

        let mut run_a = false;
        let mut run_b = false;

        // Here the type-annotation is actually legitimately needed
        let a: Option<_>   = Unit::unit(1i32);
        let b: Option<i32> = None;

        let r_a = MonadMut::bind(a, |j| {
            run_a = true;

            Unit::unit(j + 1)
        });
        let r_b = MonadMut::bind(b, |j| {
            run_b = true;

            Unit::unit(j + 2)
        });

        assert_eq!(r_a, Some(2));
        assert_eq!(run_a, true);
        assert_eq!(r_b, None);
        assert_eq!(run_b, false);
    }
}
