use vstd::prelude::*;

verus! {

pub open spec fn subst(x: Var, s: Tm, t: Tm) -> Tm
    decreases t
{
    match t {
        Tm::Var { x: y } => if x == y { s } else { t },
        Tm::Abs { x: y, ty, body } =>
            if x == y {
                t  // x is bound
            } else {
                Tm::Abs { x: y, ty, body: Box::new(subst(x, s, *body)) }
            },
        Tm::App { t1, t2 } =>
            Tm::App { t1: Box::new(subst(x, s, *t1)), t2: Box::new(subst(x, s, *t2)) },
        Tm::Tru => Tm::Tru,
        Tm::Fls => Tm::Fls,
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
    TArrow { t1: Box<Ty>, t2: Box<Ty> },
}

pub enum Tm {
    Var { x: Var },
    Abs { x: Var, ty: Ty, body: Box<Tm> },
    App { t1: Box<Tm>, t2: Box<Tm> },
    Tru,
    Fls,
}

pub open spec fn step_once(t: Tm) -> Option<Tm>
    decreases t
{
    match t {
        Tm::App { t1, t2 } => {
            match *t1 {
                Tm::Abs { x, ty: _, body } if value(*t2) =>
                    // Beta reduction
                    Option::Some(subst(x, *t2, *body)),
                _ => {
                    if !value(*t1) {
                        // Step the function
                        match step_once(*t1) {
                            Option::Some(t1_prime) =>
                                Option::Some(Tm::App { t1: Box::new(t1_prime), t2: t2 }),
                            Option::None => Option::None,
                        }
                    } else if !value(*t2) {
                        // Step the argument
                        match step_once(*t2) {
                            Option::Some(t2_prime) =>
                                Option::Some(Tm::App { t1: t1, t2: Box::new(t2_prime) }),
                            Option::None => Option::None,
                        }
                    } else {
                        Option::None
                    }
                }
            }
        }
        _ => Option::None,
    }
}


pub open spec fn eval(t: Tm, fuel: nat) -> Tm
    decreases fuel
{
    if fuel == 0 {
        t
    } else {
        match step_once(t) {
            Option::Some(t_prime) => eval(t_prime, (fuel - 1) as nat),
            Option::None => t,
        }
    }
}

} // verus!