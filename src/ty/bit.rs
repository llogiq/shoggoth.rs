use ty::{
    Tm,
    Ty,
};

/// Type-level bits
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Bit
{}

/// ```ignore
/// ---------
/// Bit :: Ty
/// ```
impl
    Ty
for
    Bit
{}



/// Type-level bit `0`
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    _0
{}

/// ```ignore
/// --------
/// 0 : Bit
/// ```
impl
    Tm<Bit>
for
    _0
{}



/// Type-level bit `1`
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    _1
{}

/// ```ignore
/// -------
/// 1 : Bit
/// ```
impl
    Tm<Bit>
for
    _1
{}
