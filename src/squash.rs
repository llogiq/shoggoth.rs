/// Evidence of a proposition where the witness has been forgotten
pub struct Squash<Sized? P>(());

impl<Sized? P> Squash<P> {
    /// Construct a `Squash` given a reference to a witness
    #[inline]
    pub fn new(_witness: &P) -> Squash<P> { Squash(()) }
}
