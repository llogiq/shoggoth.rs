pub use self::bit::{
    Bit,
    _0,
    _1,
};
pub use self::bool::{
    And,
    Bool,
    FF,
    If,
    Not,
    Or,
    TT,
};
pub use self::check::{
    Tm,
};
pub use self::kind::{
    Ty,
};
pub use self::hlist::{
    TmExt,
    TmPre,
};
pub use self::infer::{
    Infer,
};
pub use self::list::{
    List,
};
pub use self::op::{
    Ap,
    Ap1,
    AppEval,
    Arrow,
    Ar,
    Ar0,
    Ar1,
    Cmp,
    Eval,
    ProjDoms,
    ProjCods,
    IsArrow,
    Thunk,
};
pub use self::star::{
    ApT,
    ApT1,
    Lift,
    LiftMany,
    Lower,
    LowerMany,
    Star,
};
pub use self::witness::{
    Witness,
};
pub use self::zipper::{
    Get,
    Left,
    MkZipper,
    Put,
    Right,
    Unzip,
    Zip,
    Zipper,
};

mod bit;
mod bool;
mod check;
mod hlist;
mod kind;
mod op;
mod star;
mod witness;

/// Type-level type-inferrable terms
pub mod infer;

/// Type-level binary integers
pub mod int;

/// Type-level lists
pub mod list;

/// Type-level natural numbers
pub mod nat;

/// Type-level zippers for lists
pub mod zipper;

/// 0
pub type     _0b = bit::_0;
/// 2^0
pub type     _1b = bit::_1;
/// 2^1
pub type     _2b = (    _1b, _0b);
/// 2^2
pub type     _4b = (    _2b, _0b);
/// 2^3
pub type     _8b = (    _4b, _0b);
/// 2^4
pub type    _16b = (    _8b, _0b);
/// 2^5
pub type    _32b = (   _16b, _0b);
/// 2^6
pub type    _64b = (   _32b, _0b);
/// 2^7
pub type   _128b = (   _64b, _0b);
/// 2^8
pub type   _256b = (  _128b, _0b);
/// 2^9
pub type   _512b = (  _256b, _0b);
/// 2^10
pub type  _1024b = (  _512b, _0b);
/// 2^11
pub type  _2048b = ( _1024b, _0b);
/// 2^12
pub type  _4096b = ( _2048b, _0b);
/// 2^13
pub type  _8192b = ( _4096b, _0b);
/// 2^14
pub type _16384b = ( _8192b, _0b);
/// 2^15
pub type _32768b = (_16384b, _0b);
/// 2^16
pub type _65536b = (_32768b, _0b);
