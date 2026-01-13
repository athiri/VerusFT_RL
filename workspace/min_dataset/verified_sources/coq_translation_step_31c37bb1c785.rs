use vstd::prelude::*;

verus! {

pub open spec fn subst(x: Var, s: Tm, t: Tm) -> Tm
    decreases t
{
    match t {
        Tm::Var { x: y } => if x == y { s } else { t },
        Tm::Abs { x: y, ty, body } =>
            if x == y {
                t  // x is bound, no substitution in body
            } else {
                Tm::Abs { x: y, ty, body: Box::new(subst(x, s, *body)) }
            },
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


pub open spec fn step(t1: Tm, t2: Tm) -> bool
    decreases t1
{
    match t1 {
        // ST_AppAbs: (\x:T.t) v --> [x:=v]t (beta reduction)
        Tm::App { t1: func, t2: arg } => {
            match *func {
                Tm::Abs { x, ty: _, body } if value(*arg) =>
                    t2 == subst(x, *arg, *body),
                _ => {
                    // ST_App1: t1 --> t1' => t1 t2 --> t1' t2
                    if !value(*func) {
                        exists|func_prime: Tm| step(*func, func_prime) &&
                            t2 == Tm::App { t1: Box::new(func_prime), t2: arg }
                    }
                    // ST_App2: t2 --> t2' => v1 t2 --> v1 t2'
                    else if value(*func) && !value(*arg) {
                        exists|arg_prime: Tm| step(*arg, arg_prime) &&
                            t2 == Tm::App { t1: func, t2: Box::new(arg_prime) }
                    }
                    else {
                        false
                    }
                }
            }
        }
        // ST_IfTrue
        Tm::Ite { cond, then_br, else_br } => {
            match *cond {
                Tm::Tru => t2 == *then_br,
                Tm::Fls => t2 == *else_br,
                // ST_If: cond --> cond'
                _ => exists|cond_prime: Tm| step(*cond, cond_prime) &&
                    t2 == Tm::Ite { cond: Box::new(cond_prime), then_br, else_br }
            }
        }
        _ => false,
    }
}

} // verus!