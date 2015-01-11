pub use self::bool::{
    And,
    Bool,
    False,
    If,
    Not,
    Or,
    True,
};

pub use self::fin::{
    Fin,
    FZ,
    FS,
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
    LT,
    LTEq,
    Min,
    Mul,
    Nat,
    Pred,
    Sub,
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
mod fin;
mod fun;
mod list;
mod nat;
