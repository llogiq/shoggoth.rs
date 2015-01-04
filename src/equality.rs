/// Evidence of a proposition where the witness has been forgotten
pub struct Squash<Sized? P>(());

impl<Sized? P> Squash<P> {
    /// Construct a `Squash` given a reference to a witness
    #[allow(unused_variables)]
    pub fn new(witness: &P) -> Squash<P> { Squash(()) }
}

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
    pub fn refl() -> Id<A, A> {
        Id(box Refl)
    }
}

impl<A, B> Id<A, B> {
    /// Construct a `Squash` from a type equality proof.
    #[inline]
    pub fn squash(&self) -> Squash<Id<A, B>> {
        Squash::new(self)
    }
}

/// The `Is` trait acts like a type equality predicate
pub trait Is<A>: Sized {
    /// On demand, provide evidence of the truth of `Is<A>` in terms
    /// of provable type-equality of `A` and `Self`. The obligation to
    /// define this method keeps the trait from being implemented in
    /// other crates in violation of the intended semantics.
    #[inline]
    fn completeness(&self) -> Squash<Id<A, Self>>;
}

impl<A> Is<A> for A {
    #[inline]
    fn completeness(&self) -> Squash<Id<A, A>> { Id::refl().squash() }
}
