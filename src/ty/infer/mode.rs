pub trait
    Mode
{}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Show)]
pub enum
    Constant
{}
    
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Show)]
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
