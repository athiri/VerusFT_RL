use vstd::prelude::*;

verus! {

pub type Var = nat;

pub enum Ty {
    TBool,
    TArrow { t1: Box<Ty>, t2: Box<Ty> },  // Function type T1 -> T2
}

pub enum Tm {
    Var { x: Var },                                    // Variable
    Abs { x: Var, ty: Ty, body: Box<Tm> },            // Lambda abstraction \x:T.t
    App { t1: Box<Tm>, t2: Box<Tm> },                 // Application t1 t2
    Tru,                                               // true
    Fls,                                               // false
    Ite { cond: Box<Tm>, then_br: Box<Tm>, else_br: Box<Tm> },  // if-then-else
}


pub open spec fn free_in(x: Var, t: Tm) -> bool
    decreases t
{
    match t {
        Tm::Var { x: y } => x == y,
        Tm::Abs { x: y, ty: _, body } => x != y && free_in(x, *body),
        Tm::App { t1, t2 } => free_in(x, *t1) || free_in(x, *t2),
        Tm::Tru => false,
        Tm::Fls => false,
        Tm::Ite { cond, then_br, else_br } =>
            free_in(x, *cond) || free_in(x, *then_br) || free_in(x, *else_br),
    }
}

} // verus!