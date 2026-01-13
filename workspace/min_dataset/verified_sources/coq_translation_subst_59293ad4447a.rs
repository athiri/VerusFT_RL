use vstd::prelude::*;

verus! {

pub type Var = nat;

pub enum Ty {
    TBool,
    TArrow { t1: Box<Ty>, t2: Box<Ty> },
}

pub enum Tm {
    Var { x: Var },
    Abs { x: Var, ty: Ty, body: Box<Tm> },
    App { t1: Box<Tm>, t2: Box<Tm> },
    Tru,
    Fls,
    Ite { cond: Box<Tm>, then_br: Box<Tm>, else_br: Box<Tm> },
}


pub open spec fn subst(x: Var, s: Tm, t: Tm) -> Tm
    decreases t
{
    match t {
        Tm::Var { x: y } => if x == y { s } else { t },
        Tm::Abs { x: y, ty, body } =>
            if x == y { t }
            else { Tm::Abs { x: y, ty, body: Box::new(subst(x, s, *body)) } },
        Tm::App { t1, t2 } =>
            Tm::App { t1: Box::new(subst(x, s, *t1)), t2: Box::new(subst(x, s, *t2)) },
        Tm::Tru => Tm::Tru,
        Tm::Fls => Tm::Fls,
        Tm::Ite { cond, then_br, else_br } =>
            Tm::Ite {
                cond: Box::new(subst(x, s, *cond)),
                then_br: Box::new(subst(x, s, *then_br)),
                else_br: Box::new(subst(x, s, *else_br)),
            },
    }
}

} // verus!