use ac_library::Monoid;
use std::convert::Infallible;
use std::marker::PhantomData;

macro_rules! impl_monoid_tuple {
    ( $typename:ident -> $( $name:ident : $idx:tt ),+) => {
        pub struct $typename<$($name,)+>(Infallible, PhantomData<fn() -> ($($name,)+)>);
        impl<$($name: Monoid),+> Monoid for $typename<$($name,)+> {
            type S = ($($name::S,)+);

            fn identity() -> Self::S {
                ($($name::identity(),)+)
            }

            fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
                ($(
                    $name::binary_operation(&a.$idx, &b.$idx),
                )+)
            }
        }
    };
}

impl_monoid_tuple!(TupleMonoid2 -> M1:0, M2:1);
impl_monoid_tuple!(TupleMonoid3 -> M1:0, M2:1, M3:2);
impl_monoid_tuple!(TupleMonoid4 -> M1:0, M2:1, M3:2, M4:3);
impl_monoid_tuple!(TupleMonoid5 -> M1:0, M2:1, M3:2, M4:3, M5:4);
impl_monoid_tuple!(TupleMonoid6 -> M1:0, M2:1, M3:2, M4:3, M5:4, M6:5);

// example (ABC357F)
#[allow(dead_code)]
mod example {
    use super::*;
    use ac_library::MapMonoid;
    use itertools::Itertools;
    type Mint = ac_library::ModInt998244353;

    struct MintMonoid;
    impl Monoid for MintMonoid {
        type S = Mint;
        fn identity() -> Self::S {
            Mint::new(0)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a + b
        }
    }

    type M = TupleMonoid4<MintMonoid, MintMonoid, MintMonoid, MintMonoid>;

    struct MM;
    impl MapMonoid for MM {
        type M = M;
        type F = (i64, i64);
        fn identity_map() -> Self::F {
            (0, 0)
        }
        fn mapping(
            &(x, y): &Self::F,
            &(a, b, ab, len): &<Self::M as Monoid>::S,
        ) -> <Self::M as Monoid>::S {
            (
                a + len * x,
                b + len * y,
                ab + b * x + a * y + len * x * y,
                len,
            )
        }
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            (f.0 + g.0, f.1 + g.1)
        }
    }

    fn solve(
        a: &Vec<i64>,
        b: &Vec<i64>,
        queries: &Vec<(usize, usize, usize, i64)>,
    ) -> Vec<Mint> {
        let mut st = ac_library::LazySegtree::<MM>::from(
            a.iter()
                .zip(b.iter())
                .map(|(&x, &y)| {
                    (Mint::new(x), Mint::new(y), Mint::new(x * y), Mint::new(1))
                })
                .collect_vec(),
        );

        let mut ans = vec![];
        for &(t, l, r, x) in queries {
            if t == 1 {
                st.apply_range(l..=r, (x, 0))
            } else if t == 2 {
                st.apply_range(l..=r, (0, x))
            } else {
                let (_, _, ab, _) = st.prod(l..=r);
                ans.push(ab)
            }
        }
        ans
    }
}
