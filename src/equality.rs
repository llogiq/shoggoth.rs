use super::{
    Squash,
};

/// The `Term` trait classifies proof terms for type quality
trait IdTerm<A, B> {}

/// Evidence of type equality where the proof term is existentially quantified
pub struct Id<A, B>(Box<IdTerm<A, B> + 'static>);

/// Canonical proof term for type equality
struct Refl<A>;
impl<A> IdTerm<A, A> for Refl<A> {}

impl<A> Id<A, A> {
    /// Construct a proof that type `A` is equal to itself
    #[inline]
    pub fn refl() -> Id<A, A> { Id(box Refl) }
}

impl<A, B> Id<A, B> {
    /// Construct a `Squash` from a type equality proof.
    #[inline]
    pub fn squash(&self) -> Squash<Id<A, B>> { Squash::new(self) }
}

/// The `Is` trait acts like a type equality predicate
pub trait Is<A> {
    /// On demand, provide evidence of the truth of `Is<A>` in terms
    /// of provable type-equality of `A` and `Self`. The obligation to
    /// define this method keeps the trait from being implemented in
    /// other crates in violation of the intended semantics.
    #[inline]
    fn completeness(&self) -> Squash<Id<A, Self>>;

    /// Given `X: Is<Y>` and `x: X`, this method will safely coerce an
    /// `x` to type `Y`. The safety comes from the fact that the only
    /// time the bound `X: Is<Y>` holds is when `X` and `Y` are the
    /// same type (determined statically).
    #[inline]
    fn coerce(self) -> A where Self: Sized {
        * unsafe { ::std::mem::transmute::<_, Box<_>>(box self) }
    }
}

impl<A> Is<A> for A {
    #[inline]
    fn completeness(&self) -> Squash<Id<A, A>> { Id::refl().squash() }
}
