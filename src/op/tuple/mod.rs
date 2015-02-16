mod boilerplate;

pub trait IsComposite: Sized {
    type H;
    type T;

    fn pair(self) -> (Self::H, Self::T);

    #[inline]
    fn head(self) -> Self::H {
        self.pair().0
    }

    #[inline]
    fn tail(self) -> Self::T {
        self.pair().1
    }
}

pub struct At<N>;

pub struct Drop<N>;

pub struct Fill<N>;

pub struct Filter<Fx>;

pub struct FoldLeft<FxNil, FxSnoc>;

pub struct FoldRight<FxNil, FxCons>;

pub struct Head;

pub struct Init;

pub struct Last;

pub struct Length;

pub struct Map<Fx>;

pub struct MapFlat<Fx>;

pub struct Permutations;

pub struct Prepend<Xs>;

pub struct PrependReverse<Xs>;

pub struct ReduceLeft<Fx>;

pub struct ReduceRight<Fx>;

pub struct ReplaceAt<N, X>;

pub struct Reverse;

pub struct RotateLeft<N>;

pub struct RotateRight<N>;

pub struct Split<N>;

pub struct SplitReverse;

pub struct Tail;

pub struct Take<N>;

pub struct ZipApply<Fx, Xs>;

pub struct ZipConst;
