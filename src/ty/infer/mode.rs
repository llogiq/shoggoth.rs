/// Predicate classifying Infer modes
#[rustc_on_unimplemented = "`{Self}` is not a valid Infer mode"]
pub trait Mode {}

/// Infer mode for operations
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Constant {}
impl Mode for Thunk {}

/// Infer mode for thunks
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Thunk {}
impl Mode for Constant {}
