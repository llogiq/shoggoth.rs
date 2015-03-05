use std::iter::*;
use std::marker::*;

pub type Node<'a, T> = Result<T, &'a Emit<Code = T>>;

pub trait Emit {
    type Code;
    fn emit<'a>(&'a self, out: &mut Vec<Node<'a, Self::Code>>);
}

#[inline]
fn step<'a, T>(out: &mut Vec<Node<'a, T>>) -> Option<T> {
    loop { match out.pop() {
        Some(Ok (code)) => { return Some(code) }
        Some(Err(data)) => { data.emit(out) }
        _ => { return None }
    }}
}

#[inline]
fn comp<'a, T, R: 'a>(data: &'a T) -> Box<Iterator<Item = R> + 'a> where T: Emit<Code = R> {
    box Unfold::new(vec![Err(data as &'a Emit<Code = R>)], step)
}

impl<A, Rec> Emit for PhantomData<A> where A: Emit<Code = Rec> {
    type Code = Rec;
    #[inline]
    fn emit<'a>(&'a self, out: &mut Vec<Node<'a, Rec>>) {
        unsafe { ::std::mem::transmute::<&PhantomData<A>, &A>(self) }.emit(out);
    }
}

pub trait Exec {
    type Value;
    fn exec(&self, stack: &mut Vec<Self::Value>);
}

pub struct Recursive;
pub struct Streaming;

pub trait Mode: MarkerTrait {}
impl Mode for Recursive {}
impl Mode for Streaming {}

pub trait Reifies<M: Mode = Recursive> {
    type Output;
    fn reflect(&self) -> Self::Output;
}

impl<A, C, V: Sized> Reifies<Streaming> for A where A: Emit<Code = C>, C: Exec<Value = V> {
    type Output = V;
    #[inline]
    fn reflect(&self) -> V {
        let stack = &mut vec![];
        for code in comp(self) { code.exec(stack) }
        stack.pop().unwrap()
    }
}
