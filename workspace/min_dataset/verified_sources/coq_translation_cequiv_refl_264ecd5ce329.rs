use vstd::prelude::*;

verus! {

pub open spec fn store_get(st: Store, x: Var) -> int {
    if st.dom().contains(x) { st[x] } else { 0 }
}


pub open spec fn beval(b: BExp, st: Store) -> bool
    decreases b
{
    match b {
        BExp::BTrue => true,
        BExp::BFalse => false,
        BExp::BEq { a1, a2 } => aeval(*a1, st) == aeval(*a2, st),
        BExp::BLe { a1, a2 } => aeval(*a1, st) <= aeval(*a2, st),
        BExp::BNot { b1 } => !beval(*b1, st),
        BExp::BAnd { b1, b2 } => beval(*b1, st) && beval(*b2, st),
    }
}

pub open spec fn aeval(a: AExp, st: Store) -> int
    decreases a
{
    match a {
        AExp::ANum { n } => n,
        AExp::AId { x } => store_get(st, x),
        AExp::APlus { a1, a2 } => aeval(*a1, st) + aeval(*a2, st),
        AExp::AMinus { a1, a2 } => aeval(*a1, st) - aeval(*a2, st),
        AExp::AMult { a1, a2 } => aeval(*a1, st) * aeval(*a2, st),
    }
}

pub open spec fn store_update(st: Store, x: Var, v: int) -> Store {
    st.insert(x, v)
}


pub open spec fn ceval(fuel: nat, c: Com, st: Store) -> Option<Store>
    decreases fuel, c
{
    if fuel == 0 {
        Option::<Store>::None
    } else {
        let fuel1 = (fuel - 1) as nat;
        match c {
            Com::CSkip => Option::Some(st),
            Com::CAsgn { x, a } => Option::Some(store_update(st, x, aeval(*a, st))),
            Com::CSeq { c1, c2 } => {
                match ceval(fuel1, *c1, st) {
                    Option::None => Option::<Store>::None,
                    Option::Some(st1) => ceval(fuel1, *c2, st1),
                }
            }
            Com::CIf { b, ct, cf } => {
                if beval(*b, st) {
                    ceval(fuel1, *ct, st)
                } else {
                    ceval(fuel1, *cf, st)
                }
            }
            Com::CWhile { b, body } => {
                if beval(*b, st) {
                    match ceval(fuel1, *body, st) {
                        Option::None => Option::<Store>::None,
                        Option::Some(st1) => ceval(fuel1, Com::CWhile { b: Box::new(*b), body: Box::new(*body) }, st1),
                    }
                } else {
                    Option::Some(st)
                }
            }
        }
    }
}


pub type Var = nat;

pub enum AExp {
    ANum { n: int },
    AId { x: Var },
    APlus { a1: Box<AExp>, a2: Box<AExp> },
    AMinus { a1: Box<AExp>, a2: Box<AExp> },
    AMult { a1: Box<AExp>, a2: Box<AExp> },
}

pub enum BExp {
    BTrue,
    BFalse,
    BEq { a1: Box<AExp>, a2: Box<AExp> },
    BLe { a1: Box<AExp>, a2: Box<AExp> },
    BNot { b1: Box<BExp> },
    BAnd { b1: Box<BExp>, b2: Box<BExp> },
}

pub enum Com {
    CSkip,
    CAsgn { x: Var, a: Box<AExp> },
    CSeq { c1: Box<Com>, c2: Box<Com> },
    CIf { b: Box<BExp>, ct: Box<Com>, cf: Box<Com> },
    CWhile { b: Box<BExp>, body: Box<Com> },
}

pub type Store = Map<Var, int>;

pub open spec fn cequiv(c1: Com, c2: Com) -> bool {
    forall|fuel: nat, st: Store| ceval(fuel, c1, st) == ceval(fuel, c2, st)
}


pub proof fn cequiv_refl(c: Com)
    ensures cequiv(c, c)
{
}

} // verus!