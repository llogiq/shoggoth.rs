/// Predicate classifying Infer modes
#[rustc_on_unimplemented = "`{Self}` is not a valid Infer mode"]
pub trait
    Mode
{}

/// Infer mode for operations
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Constant
{}

/// Infer mode for thunks
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Thunk
{}

impl
    Mode
for
    Thunk
{}

impl
    Mode
for
    Constant
{}
