use ty::{
    Tm,
    Ty,
};

/// Type-level bits
pub enum Bit {}

/// ```
/// ---------
/// Bit :: Ty
/// ```
impl Ty for Bit {}



/// Type-level bit `0`
pub enum _0 {}

/// ```
/// --------
/// 0 : Bit
/// ```
impl Tm<Bit> for _0 {}



/// Type-level bit `1`
pub enum _1 {}

/// ```
/// -------
/// 1 : Bit
/// ```
impl Tm<Bit> for _1 {}
