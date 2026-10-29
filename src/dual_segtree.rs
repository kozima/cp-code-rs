/// Dual segment tree
/// Use LazySegtree::<Update> for range update and point query
use ac_library::Monoid;
use ac_library::lazysegtree::MapMonoid;

pub struct LeftZeroBand<T: Default>(
    std::convert::Infallible,
    std::marker::PhantomData<fn() -> T>,
);
impl<T: Default + Clone> Monoid for LeftZeroBand<T> {
    type S = T;
    fn identity() -> Self::S {
        T::default()
    }
    fn binary_operation(a: &Self::S, _b: &Self::S) -> Self::S {
        a.clone()
    }
}

pub struct Update;
impl MapMonoid for Update {
    type M = LeftZeroBand<(usize, usize)>;
    type F = Option<(usize, usize)>;
    fn identity_map() -> Self::F {
        None
    }
    fn mapping(
        f: &Self::F,
        x: &<Self::M as Monoid>::S,
    ) -> <Self::M as Monoid>::S {
        f.unwrap_or(*x)
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f.or(*g)
    }
}
