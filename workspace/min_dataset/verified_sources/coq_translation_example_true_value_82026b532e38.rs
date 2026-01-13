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

pub open spec fn value(t: Tm) -> bool {
    match t {
        Tm::Abs { .. } => true,
        Tm::Tru => true,
        Tm::Fls => true,
        _ => false,
    }
}


pub proof fn example_true_value()
    ensures value(Tm::Tru)
{
}

} // verus!