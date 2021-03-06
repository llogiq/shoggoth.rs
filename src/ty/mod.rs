pub use self::bit::{
    _0,
    _1,
};
pub use self::bool::{
    And,
    FF,
    If,
    Not,
    Or,
    TT,
};
pub use self::hlist::{
    TmExt,
    TmPre,
};
pub use self::infer::{
    Infer,
};
pub use self::lens::{
    MkStore,
    Over,
    View,
};
pub use self::op::{
    Ap,
    Ap1,
    AppEval,
    Cmp,
    Cmp1,
    Const,
    Eval,
    Eval1,
    ProjDoms,
    ProjCods,
    Id,
    Thunk,
};
pub use self::witness::{
    Witness,
};
pub use self::zipper::{
    MkZipper,
    Unzip,
    Zip,
    Zipper,
};

mod bit;
mod bool;
mod hlist;
mod op;
mod witness;

/// Type-level type-inferrable terms
pub mod infer;

// /// Type-level binary integers
// pub mod int;

/// Type-level lenses
pub mod lens;

/// Type-level lists
pub mod list;

// /// Type-level natural numbers
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
