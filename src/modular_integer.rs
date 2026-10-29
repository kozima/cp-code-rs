// modular combination

pub trait ModIntType:
    Copy
    + PartialEq
    + std::fmt::Debug
    + std::ops::Add<Output = Self>
    + std::ops::Mul<Output = Self>
{
    fn modulus() -> u32;
    fn new(v: i64) -> Self;
    fn inv(self) -> Self;
}

impl<const MOD: u32> ModIntType for mod_int::ModInt<MOD> {
    fn modulus() -> u32 {
        MOD
    }
    fn new(v: i64) -> Self {
        mod_int::ModInt::<MOD>::new(v)
    }
    fn inv(self) -> Self {
        self.inv()
    }
}

impl<M: ac_library::Modulus> ModIntType for ac_library::StaticModInt<M> {
    fn modulus() -> u32 {
        M::VALUE
    }
    fn new(v: i64) -> Self {
        ac_library::StaticModInt::<M>::new(v)
    }
    fn inv(self) -> Self {
        self.inv()
    }
}

pub mod mod_comb {
    use super::ModIntType;

    #[derive(Clone, Hash, PartialEq, Eq, Debug)]
    pub struct ModComb<I: ModIntType> {
        fact: Vec<I>,
        facti: Vec<I>,
        size: usize,
    }

    impl<I: ModIntType> ModComb<I> {
        pub fn new() -> ModComb<I> {
            ModComb {
                fact: vec![],
                facti: vec![],
                size: 0,
            }
        }

        fn ensure_size(&mut self, n: usize) {
            if n > self.size {
                let n = n.max(self.size * 2).min(I::modulus() as usize);
                let osize = self.fact.len();
                self.fact.resize(n, I::new(1));
                self.facti.resize(n, I::new(1));
                for i in (osize.max(1))..n {
                    self.fact[i] = self.fact[i - 1] * I::new(i as i64);
                }
                self.facti[n - 1] = self.fact[n - 1].inv();
                for i in (osize..(n - 1)).rev() {
                    self.facti[i] = self.facti[i + 1] * I::new(i as i64 + 1);
                }
                self.size = n;
            }
        }

        pub fn factorial(&mut self, n: i64) -> I {
            if n < I::modulus() as i64 {
                self.ensure_size(((n as usize) + 1).min(I::modulus() as usize));
                self.fact[n as usize]
            } else {
                I::new(0)
            }
        }

        pub fn factorial_inverse(&mut self, n: i64) -> I {
            if n < I::modulus() as i64 {
                self.ensure_size(((n as usize) + 1).min(I::modulus() as usize));
                self.facti[n as usize]
            } else {
                panic!()
            }
        }

        pub fn combination(&mut self, n: i64, k: i64) -> I {
            if n < 0 || k < 0 || k > n {
                I::new(0)
            } else if n < I::modulus() as i64 {
                self.factorial(n)
                    * self.factorial_inverse(k)
                    * self.factorial_inverse(n - k)
            } else {
                let q1 = n / I::modulus() as i64;
                let r1 = n % I::modulus() as i64;
                let q2 = k / I::modulus() as i64;
                let r2 = k % I::modulus() as i64;
                let q3 = (n - k) / I::modulus() as i64;
                if q1 == q2 + q3 {
                    // Lucas' theorem
                    self.combination(r1, r2) * self.combination(q1, q2)
                } else {
                    I::new(0)
                }
            }
        }

        pub fn permutation(&mut self, n: i64, k: i64) -> I {
            if n < 0 || k < 0 || k > n {
                I::new(0)
            } else if n < I::modulus() as i64 {
                self.factorial(n) * self.factorial_inverse(n - k)
            } else if n / I::modulus() as i64 != (n - k) / I::modulus() as i64 {
                I::new(0)
            } else {
                let r1 = n % I::modulus() as i64;
                let r2 = (n - k) % I::modulus() as i64;
                assert!(r1 >= r2);
                self.factorial(r1) * self.factorial_inverse(r2)
            }
        }
    }
}

// modular integer arithmetic

pub mod mod_int {
    #[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
    pub struct ModInt<const MOD: u32> {
        value: u32,
    }

    impl<const MOD: u32> ModInt<MOD> {
        fn normalize(v: i64) -> u32 {
            if 0 <= v && v < MOD as i64 {
                v as u32
            } else {
                let v1 = v % MOD as i64;
                (if v1 < 0 { v1 + MOD as i64 } else { v1 }) as u32
            }
        }
        pub fn new(v: i64) -> ModInt<MOD> {
            ModInt {
                value: Self::normalize(v),
            }
        }
        pub fn inv(self) -> Self {
            let mut x = MOD as i64;
            let mut y = self.value as i64;
            let mut p = 1;
            let mut q = 0;
            let mut r = 0;
            let mut s = 1;
            while y != 0 {
                let u = x / y;
                let x0 = y;
                y = x - y * u;
                x = x0;
                let r0 = p - r * u;
                let s0 = q - s * u;
                p = r;
                r = r0;
                q = s;
                s = s0;
            }
            ModInt::new(q)
        }
        pub fn pow(&self, n: i64) -> Self {
            if n == i64::MIN {
                let inv = self.inv();
                inv * inv.pow(i64::MAX)
            } else if n < 0 {
                self.inv().pow(-n)
            } else {
                let mut p = *self;
                let mut result = Self::new(1);
                let mut e = n;
                while e > 0 {
                    if e % 2 == 1 {
                        result *= p;
                    }
                    p *= p;
                    e /= 2;
                }
                result
            }
        }
    }

    impl<const MOD: u32> std::fmt::Display for ModInt<MOD> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    impl<const MOD: u32> num::traits::One for ModInt<MOD> {
        fn one() -> ModInt<MOD> {
            ModInt { value: 1 }
        }
    }

    impl<const MOD: u32> num::traits::Zero for ModInt<MOD> {
        fn zero() -> ModInt<MOD> {
            ModInt { value: 0 }
        }
        fn is_zero(&self) -> bool {
            self.value == 0
        }
    }

    impl<const MOD: u32> core::ops::Add<Self> for ModInt<MOD> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            let value = self.value as i64 + rhs.value as i64;
            let value = (if value < MOD as i64 {
                value
            } else {
                value - MOD as i64
            }) as u32;
            ModInt { value }
        }
    }

    impl<const MOD: u32> core::ops::Add<i64> for ModInt<MOD> {
        type Output = Self;
        fn add(self, rhs: i64) -> Self {
            self + ModInt::new(rhs)
        }
    }

    impl<const MOD: u32> core::ops::Sub<Self> for ModInt<MOD> {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self {
            let value = self.value as i64 - rhs.value as i64;
            let value = (if value >= 0 {
                value
            } else {
                value + MOD as i64
            }) as u32;
            ModInt { value }
        }
    }

    impl<const MOD: u32> core::ops::Sub<i64> for ModInt<MOD> {
        type Output = Self;
        fn sub(self, rhs: i64) -> Self {
            self - ModInt::new(rhs)
        }
    }

    impl<const MOD: u32> core::ops::AddAssign<Self> for ModInt<MOD> {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }

    impl<const MOD: u32> core::ops::AddAssign<i64> for ModInt<MOD> {
        fn add_assign(&mut self, rhs: i64) {
            *self = *self + rhs
        }
    }

    impl<const MOD: u32> core::ops::Mul<Self> for ModInt<MOD> {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self {
            Self::new(self.value as i64 * rhs.value as i64)
        }
    }

    impl<const MOD: u32> core::ops::MulAssign<Self> for ModInt<MOD> {
        fn mul_assign(&mut self, rhs: Self) {
            *self = *self * rhs
        }
    }

    impl<const MOD: u32> core::ops::Div<Self> for ModInt<MOD> {
        type Output = Self;
        fn div(self, rhs: Self) -> Self {
            self * rhs.inv()
        }
    }

    impl<const MOD: u32> core::ops::Div<i64> for ModInt<MOD> {
        type Output = Self;
        fn div(self, rhs: i64) -> Self {
            self / ModInt::new(rhs)
        }
    }

    impl<const MOD: u32> core::ops::DivAssign<Self> for ModInt<MOD> {
        fn div_assign(&mut self, rhs: Self) {
            *self = *self / rhs
        }
    }

    impl<const MOD: u32> std::iter::Sum<Self> for ModInt<MOD> {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            use num::traits::Zero;
            iter.fold(Self::zero(), |a, b| a + b)
        }
    }

    pub type ModComb<const MOD: u32> = super::mod_comb::ModComb<ModInt<MOD>>;
}

// test

#[cfg(test)]
mod test {
    fn test_mod_comb<I: super::ModIntType>() {
        let mut mc = super::mod_comb::ModComb::<I>::new();
        assert_eq!(mc.combination(0, 0), I::new(1));
        assert_eq!(mc.permutation(0, 0), I::new(1));

        assert_eq!(mc.combination(0, 1), I::new(0));
        assert_eq!(mc.permutation(0, 1), I::new(0));

        assert_eq!(mc.combination(1, 0), I::new(1));
        assert_eq!(mc.permutation(1, 0), I::new(1));

        assert_eq!(mc.combination(1, 1), I::new(1));
        assert_eq!(mc.permutation(1, 1), I::new(1));

        assert_eq!(mc.combination(1, 2), I::new(0));
        assert_eq!(mc.permutation(1, 2), I::new(0));

        assert_eq!(mc.combination(5, 2), I::new(10));
        assert_eq!(mc.permutation(5, 2), I::new(20));
    }

    #[test]
    fn test_ac_library_mod_comb() {
        test_mod_comb::<ac_library::ModInt998244353>();
    }

    #[test]
    fn test_mod_int_comb() {
        type MInt = super::mod_int::ModInt<998244353>;
        test_mod_comb::<MInt>();
    }
}
