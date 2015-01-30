use hlist::*;
use ty::{
    Infer,
    Tm,
};
use ty::op::{
    IsArr,
};

pub trait
    Eval<Fx>
where
    Fx: Infer,
    <Fx as Infer>::Ty: IsArr,
    Self: HList,
    Self: Tm<<<Fx as Infer>::Ty as IsArr>::Dom>,
{
    type Out: Tm<<<Fx as Infer>::Ty as IsArr>::Cod>;
}
