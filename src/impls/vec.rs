use ::{FunctorMut, HKT, MonadMut, Unit};

impl<T, U> HKT<U> for Vec<T> {
    type Result = Vec<U>;
}

impl<T, U> FunctorMut<U> for Vec<T> {
    fn map<F>(self, f: F) -> Self::Result
      where F: FnMut(T) -> U {
        self.into_iter().map(f).collect()
    }
}

impl<T> Unit for Vec<T> {
    type Inner = T;

    fn unit(t: Self::Inner) -> Self {
        vec![t]
    }
}

impl<T, U> MonadMut<U> for Vec<T> {
    fn bind<F>(self, mut f: F) -> Self::Result
      where F: FnMut(T) -> Vec<U> {
        let mut ret = Vec::with_capacity(self.len());

        for i in self.into_iter() {
            ret.extend(f(i));
        }

        ret
    }
}

#[cfg(test)]
mod test {
    use ::{Functor, Monad, Unit};

    #[test]
    fn test_map() {
        assert_eq!(Functor::map(vec![1, 2, 3], |x| x * 2), vec![1, 4, 6]);
    }

    #[test]
    fn left_identity() {
        fn f(i: u32) -> Vec<i32> {
            Unit::unit((i as i32) - 1)
        }

        // return a >>= f
        let lhs = Monad::bind(Vec::unit(1), f);
        // f a
        let rhs = f(1);

        assert_eq!(lhs, rhs);
    }

    #[test]
    fn right_identity() {
        // m >>= return
        let lhs = Monad::bind(vec![1], Unit::unit);
        // m
        let rhs = vec![1];

        assert_eq!(lhs, rhs);
    }

    #[test]
    fn associativity() {
         fn f(num: u32) -> Vec<u64> {
            Unit::unit((num + 1) as u64)
        }

        fn g(num: u64) -> Vec<u64> {
            Unit::unit(num * 2)
        }

        // (m >>= f) >>= g
        let lhs = Monad::bind(Monad::bind(vec![2], f), g);
        // m >>= (\x -> f x >> g)
        let rhs = Monad::bind(vec![2], |x| Monad::bind(f(x), g));

        assert_eq!(lhs, rhs);
    }
}
