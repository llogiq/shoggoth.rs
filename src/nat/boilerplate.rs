use bit::{
    _0,
    _1,
};
use nat::{
    Add,
    AddCarry,
    IsNat,
    IsPos,
    Nat,
    Succ,
};

// Fn: Succ ///////////////////////////////////////////////////////////////////

/// `succ(0) ==> 1`
impl Fn<(Nat<_0>,)> for Succ {
    type Output = Nat<_1>;
    #[inline]
    extern "rust-call" fn call(&self, (Nat(_0 {}),): (Nat<_0>,)) -> Nat<_1> {
        Nat(_1)
    }
}
/// `succ(1) ==> 1:0`
impl Fn<(_1,)> for Succ {
    type Output = (_1, _0);
    #[inline]
    extern "rust-call" fn call(&self, (_1 {},): (_1,)) -> (_1, _0) {
        (_1, _0)
    }
}
/// `succ(p:0) ==> p:1`
impl<P: IsPos> Fn<((P, _0),)> for Succ {
    type Output = (P, _1);
    #[inline]
    extern "rust-call" fn call(&self, ((p, _),): ((P, _0),)) -> (P, _1) {
        (p, _1)
    }
}
/// `succ(p:1) ==> succ(p):0`
impl<P: IsPos, Rec> Fn<((P, _1),)> for Succ where
    Succ: Fn<(P,), Output = Rec>,
{
    type Output = (Rec, _0);
    #[inline]
    extern "rust-call" fn call(&self, ((p, _),): ((P, _1),)) -> (Rec, _0) {
        (Succ(p), _0)
    }
}

// Fn: Add ////////////////////////////////////////////////////////////////////

/// `add(0, q) ==> q`
impl<Q: IsPos> Fn<((Nat<_0>,), (Nat<Q>,))> for Add {
    type Output = Nat<Q>;
    #[inline]
    extern "rust-call" fn call(&self, ((Nat(_0 {}),), (Nat(q),)): ((Nat<_0>,), (Nat<Q>,))) -> Nat<Q> {
        Nat(q)
    }
}
/// `add(p, 0) ==> p`
impl<P: IsPos> Fn<((Nat<P>,), (Nat<_0>,))> for Add {
    type Output = Nat<P>;
    #[inline]
    extern "rust-call" fn call(&self, ((Nat(p),), (Nat(_0 {}),)): ((Nat<P>,), (Nat<_0>,))) -> Nat<P> {
        Nat(p)
    }
}
/// `add(1, 1) ==> 1:0`
impl Fn<(_1, _1)> for Add {
    type Output = (_1, _0);
    #[inline]
    extern "rust-call" fn call(&self, (_1 {}, _1 {}): (_1, _1)) -> (_1, _0) {
        (_1, _0)
    }
}
/// `add(1, q:0) ==> q:1`
impl<Q: IsPos> Fn<(_1, (Q, _0))> for Add {
    type Output = (Q, _1);
    #[inline]
    extern "rust-call" fn call(&self, (_1 {}, (q, _0 {})): (_1, (Q, _0))) -> (Q, _1) {
        (q, _1)
    }
}
/// `add(1, q:1) ==> succ(q):0`
impl<Q: IsPos, K> Fn<(_1, (Q, _1))> for Add where
    Succ: Fn<(Q,), Output = K>
{
    type Output = (K, _0);
    #[inline]
    extern "rust-call" fn call(&self, (_1 {}, (q, _1 {})): (_1, (Q, _1))) -> (K, _0) {
        (Succ(q), _0)
    }
}
/// `add(p:0, 1) ==> p:1`
impl<P: IsPos> Fn<((P, _0), _1)> for Add {
    type Output = (P, _1);
    #[inline]
    extern "rust-call" fn call(&self, ((p, _0 {}), _1 {}): ((P, _0), _1)) -> (P, _1) {
        (p, _1)
    }
}
/// `add(p:0, q:0) ==> add(p, q):0`
impl<P: IsPos, Q: IsPos, K> Fn<((P, _0), (Q, _0))> for Add where
    Add: Fn<(P, Q), Output = K>
{
    type Output = (K, _0);
    #[inline]
    extern "rust-call" fn call(&self, ((p, _0 {}), (q, _0 {})): ((P, _0), (Q, _0))) -> (K, _0) {
        (Add(p, q), _0)
    }
}
/// `add(p:0, q:1) ==> add(p, q):1`
impl<P: IsPos, Q: IsPos, K> Fn<((P, _0), (Q, _1))> for Add where
    Add: Fn<(P, Q), Output = K>
{
    type Output = (K, _1);
    #[inline]
    extern "rust-call" fn call(&self, ((p, _0 {}), (q, _1 {})): ((P, _0), (Q, _1))) -> (K, _1) {
        (Add(p, q), _1)
    }
}
/// `add(p:1, 1) ==> succ(p):0`
impl<P: IsPos, K> Fn<((P, _1), _1)> for Add where
    Succ: Fn<(P,), Output = K>
{
    type Output = (K, _0);
    #[inline]
    extern "rust-call" fn call(&self, ((p, _1 {}), _1 {}): ((P, _1), _1)) -> (K, _0) {
        (Succ(p), _0)
    }
}
/// `add(p:1, q:0) ==> add(p, q):1`
impl<P: IsPos, Q: IsPos, K> Fn<((P, _1), (Q, _0,))> for Add where
    Add: Fn<(P, Q), Output = K>
{
    type Output = (K, _1);
    #[inline]
    extern "rust-call" fn call(&self, ((p, _1 {}), (q, _0 {})): ((P, _1), (Q, _0))) -> (K, _1) {
        (Add(p, q), _1)
    }
}
/// `add(p:1, q:1) ==> add_carry(p, q):0`
impl<P: IsPos, Q: IsPos, K> Fn<((P, _1), (Q, _1,))> for Add where
    AddCarry: Fn<(P, Q), Output = K>
{
    type Output = (K, _0);
    #[inline]
    extern "rust-call" fn call(&self, ((p, _1 {}), (q, _1 {})): ((P, _1), (Q, _1))) -> (K, _0) {
        (AddCarry(p, q), _0)
    }
}

// Fn: AddCarry ///////////////////////////////////////////////////////////////

/// `add_carry(1, 1) ==> 1:1`
impl Fn<(_1, _1)> for AddCarry {
    type Output = (_1, _1);
    #[inline]
    extern "rust-call" fn call(&self, (_1 {}, _1 {}): (_1, _1)) -> (_1, _1) {
        (_1, _1)
    }
}
/// `add_carry(1, q:0) ==> succ(q):0`
impl<Q: IsPos, K> Fn<(_1, (Q, _0))> for AddCarry where
    Succ: Fn<(Q,), Output = K>
{
    type Output = (K, _0);
    #[inline]
    extern "rust-call" fn call(&self, (_1 {}, (q, _0 {})): (_1, (Q, _0))) -> (K, _0) {
        (Succ(q), _0)
    }
}
/// `add_carry(1, q:1) ==> succ(q):1`
impl<Q: IsPos, K> Fn<(_1, (Q, _1))> for AddCarry where
    Succ: Fn<(Q,), Output = K>
{
    type Output = (K, _1);
    #[inline]
    extern "rust-call" fn call(&self, (_1 {}, (q, _1 {})): (_1, (Q, _1))) -> (K, _1) {
        (Succ(q), _1)
    }
}
/// `add_carry(p:0, 1) ==> succ(p):0`
impl<P: IsPos, K> Fn<((P, _0), _1)> for AddCarry where
    Succ: Fn<(P,), Output = K>
{
    type Output = (K, _0);
    #[inline]
    extern "rust-call" fn call(&self, ((p, _0 {},), _1 {}): ((P, _0), _1)) -> (K, _0) {
        (Succ(p), _0)
    }
}
/// `add_carry(p:0, q:0) ==> add(p, q):1`
impl<P: IsPos, Q: IsPos, K> Fn<((P, _0), (Q, _0))> for AddCarry where
    Add: Fn<(P, Q), Output = K>
{
    type Output = (K, _1);
    #[inline]
    extern "rust-call" fn call(&self, ((p, _0 {}), (q, _0 {})): ((P, _0), (Q, _0))) -> (K, _1) {
        (Add(p, q), _1)
    }
}
/// `add_carry(p:0, q:1) ==> add_carry(p, q):0`
impl<P: IsPos, Q: IsPos, K> Fn<((P, _0), (Q, _1))> for AddCarry where
    AddCarry: Fn<(P, Q), Output = K>
{
    type Output = (K, _0);
    #[inline]
    extern "rust-call" fn call(&self, ((p, _0 {}), (q, _1 {})): ((P, _0), (Q, _1))) -> (K, _0) {
        (AddCarry(p, q), _0)
    }
}
/// `add_carry(p:1, 1) ==> succ(p):1`
impl<P: IsPos, K> Fn<((P, _1), _1)> for AddCarry where
    Succ: Fn<(P,), Output = K>
{
    type Output = (K, _1);
    #[inline]
    extern "rust-call" fn call(&self, ((p, _1 {}), _1 {}): ((P, _1), _1)) -> (K, _1) {
        (Succ(p), _1)
    }
}
/// `add_carry(p:1, q:0) ==> add_carry(p, q):0`
impl<P: IsPos, Q: IsPos, K> Fn<((P, _1), (Q, _0,))> for AddCarry where
    AddCarry: Fn<(P, Q), Output = K>
{
    type Output = (K, _0);
    #[inline]
    extern "rust-call" fn call(&self, ((p, _1 {}), (q, _0 {})): ((P, _1), (Q, _0))) -> (K, _0) {
        (AddCarry(p, q), _0)
    }
}
/// `add_carry(p:1, q:1) ==> add_carry(p, q):1`
impl<P: IsPos, Q: IsPos, K> Fn<((P, _1), (Q, _1,))> for AddCarry where
    AddCarry: Fn<(P, Q), Output = K>
{
    type Output = (K, _1);
    #[inline]
    extern "rust-call" fn call(&self, ((p, _1 {}), (q, _1 {})): ((P, _1), (Q, _1))) -> (K, _1) {
        (AddCarry(p, q), _1)
    }
}

// Infix: Add /////////////////////////////////////////////////////////////////

impl<M: IsNat, N: IsNat, K: IsNat> ::std::ops::Add<Nat<N>> for Nat<M> where
    Add: Fn<(M, N), Output = K>
{
    type Output = Nat<K>;
    #[inline]
    fn add(self, rhs: Nat<N>) -> Nat<K> {
        Nat(Add(self.0, rhs.0))
    }
}
