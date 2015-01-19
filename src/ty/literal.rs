pub use hlist::{
    HC,
    HList,
    HN,
};
use ty::{
    bool,
};
use ty::bit;

pub type FF = bool::False;
pub type TT = bool::True;

pub type     _0b = bit::_0;
pub type     _1b = bit::_1;
pub type     _2b = (    _1b, _0b);
pub type     _4b = (    _2b, _0b);
pub type     _8b = (    _4b, _0b);
pub type    _16b = (    _8b, _0b);
pub type    _32b = (   _16b, _0b);
pub type    _64b = (   _32b, _0b);
pub type   _128b = (   _64b, _0b);
pub type   _256b = (  _128b, _0b);
pub type   _512b = (  _256b, _0b);
pub type  _1024b = (  _512b, _0b);
pub type  _2048b = ( _1024b, _0b);
pub type  _4096b = ( _2048b, _0b);
pub type  _8192b = ( _4096b, _0b);
pub type _16384b = ( _8192b, _0b);
pub type _32768b = (_16384b, _0b);
pub type _65536b = (_32768b, _0b);
