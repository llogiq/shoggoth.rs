/// Infer modes for controlling `AppEval` behavior
pub mod mode;

/// Predicate classifying type-level type-inferrable terms
#[rustc_on_unimplemented = "`{Self}` is not a valid type-level type-inferrable term"]
pub trait Infer {
    type Arity;
    type Mode;
}
