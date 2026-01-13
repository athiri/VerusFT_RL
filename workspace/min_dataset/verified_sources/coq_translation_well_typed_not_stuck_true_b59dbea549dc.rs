use vstd::prelude::*;

verus! {

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


pub open spec fn step(t: Tm) -> Option<Tm>
    decreases t
{
    match t {
        Tm::App { t1, t2 } => {
            match *t1 {
                Tm::Abs { x, ty: _, body } if value(*t2) =>
                    Option::Some(subst(x, *t2, *body)),
                _ => {
                    if !value(*t1) {
                        match step(*t1) {
                            Option::Some(t1p) =>
                                Option::Some(Tm::App { t1: Box::new(t1p), t2: t2 }),
                            Option::None => Option::None,
                        }
                    } else if !value(*t2) {
                        match step(*t2) {
                            Option::Some(t2p) =>
                                Option::Some(Tm::App { t1: t1, t2: Box::new(t2p) }),
                            Option::None => Option::None,
                        }
                    } else {
                        Option::None
                    }
                }
            }
        }
        Tm::Ite { cond, then_br, else_br } => {
            match *cond {
                Tm::Tru => Option::Some(*then_br),
                Tm::Fls => Option::Some(*else_br),
                _ => match step(*cond) {
                    Option::Some(cp) =>
                        Option::Some(Tm::Ite { cond: Box::new(cp), then_br, else_br }),
                    Option::None => Option::None,
                }
            }
        }
        _ => Option::None,
    }
}

pub open spec fn progress_holds(t: Tm) -> bool {
    value(t) || step(t).is_some()
}

pub open spec fn value(t: Tm) -> bool {
    match t {
        Tm::Abs { .. } => true,
        Tm::Tru => true,
        Tm::Fls => true,
        _ => false,
    }
}


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

pub open spec fn stuck(t: Tm) -> bool {
    !value(t) && step(t).is_none()
}

pub proof fn progress_true()
    ensures progress_holds(Tm::Tru)
{
    assert(value(Tm::Tru));
}


pub proof fn well_typed_not_stuck_true()
    ensures !stuck(Tm::Tru)
{
    progress_true();
}

} // verus!