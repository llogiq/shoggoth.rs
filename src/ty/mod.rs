pub use self::bool::{
    And,
    Bool,
    False,
    If,
    Not,
    Or,
    True,
};

pub use self::eq::{
    Eq,
    Id,
};

pub use self::fun::{
    Ap,
    DepFn,
    Fn,
    Val,
    val,
};

pub use self::nat::{
    Add,
    Exp,
    Fac,
    Mul,
    Nat,
    S,
    Z,
};

pub use self::list::{
    Append,
    Cons,
    Length,
    List,
    Nil,
};

mod bool;
mod eq;
mod fun;
mod list;
mod nat;
