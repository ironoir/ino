use std::rc::Rc;
use std::collections::HashMap;

// pub struct One;
// pub struct Zero;
// pub struct Cats;

#[derive(Clone)]

pub enum Sort {
    Prim(usize),
    Func(Rc<Sort>, Rc<Sort>),
    Mult(Rc<Sort>, Rc<Sort>),
}

#[derive(Clone)]
pub struct Func {
    pub name: String,
    pub sort: Rc<Sort>,
}

pub struct Context(pub HashMap<String, Rc<Sort>>);

// pub struct Term {
//     context: Context,
//     inner: TermInner
// }

pub enum TermInner {
    Var(String),
    Fun(Func, Vec<Rc<TermInner>>),
}

pub struct Equation {
    pub context: Context,
    pub left: TermInner,
    pub right: TermInner,
}

pub struct Schema {
    pub sorts: Vec<Rc<Sort>>,
    pub funcs: Vec<Func>,
    pub equations: Vec<Equation>,
}
