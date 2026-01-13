use vstd::prelude::*;

verus! {

pub open spec fn store_get(st: Store, default: int, x: Var) -> int {
    if st.dom().contains(x) { st[x] } else { default }
}


pub open spec fn beval(b: BExp, st: Store, default: int) -> bool
    decreases b
{
    match b {
        BExp::B { b } => b,
        BExp::Eq { a1, a2 } => aeval(*a1, st, default) == aeval(*a2, st, default),
        BExp::Le { a1, a2 } => aeval(*a1, st, default) <= aeval(*a2, st, default),
        BExp::Not { b1 } => !beval(*b1, st, default),
        BExp::And { b1, b2 } => beval(*b1, st, default) && beval(*b2, st, default),
    }
}

pub open spec fn aeval(a: AExp, st: Store, default: int) -> int
    decreases a
{
    match a {
        AExp::N { n } => n,
        AExp::V { x } => store_get(st, default, x),
        AExp::Plus { a1, a2 } => aeval(*a1, st, default) + aeval(*a2, st, default),
        AExp::Minus { a1, a2 } => aeval(*a1, st, default) - aeval(*a2, st, default),
    }
}

pub open spec fn store_update(st: Store, x: Var, v: int) -> Store {
    st.insert(x, v)
}


pub type Var = nat;

pub enum AExp {
    N { n: int },
    V { x: Var },
    Plus { a1: Box<AExp>, a2: Box<AExp> },
    Minus { a1: Box<AExp>, a2: Box<AExp> },
}

pub enum BExp {
    B { b: bool },
    Eq { a1: Box<AExp>, a2: Box<AExp> },
    Le { a1: Box<AExp>, a2: Box<AExp> },
    Not { b1: Box<BExp> },
    And { b1: Box<BExp>, b2: Box<BExp> },
}

pub enum Com {
    Skip,
    Assign { x: Var, a: Box<AExp> },
    Seq { c1: Box<Com>, c2: Box<Com> },
    If { b: Box<BExp>, ct: Box<Com>, cf: Box<Com> },
    While { b: Box<BExp>, body: Box<Com> },
}

pub type Store = Map<Var, int>;

pub open spec fn ceval_step(fuel: nat, c: Com, st: Store, default: int) -> Option<Store>
    decreases fuel, c
{
    if fuel == 0 {
        Option::<Store>::None
    } else {
        let fuel1 = (fuel - 1) as nat;
        match c {
            Com::Skip => Option::Some(st),
            Com::Assign { x, a } => Option::Some(store_update(st, x, aeval(*a, st, default))),
            Com::Seq { c1, c2 } => {
                match ceval_step(fuel1, *c1, st, default) {
                    Option::None => Option::<Store>::None,
                    Option::Some(st1) => ceval_step(fuel1, *c2, st1, default),
                }
            }
            Com::If { b, ct, cf } => {
                if beval(*b, st, default) {
                    ceval_step(fuel1, *ct, st, default)
                } else {
                    ceval_step(fuel1, *cf, st, default)
                }
            }
            Com::While { b, body } => {
                if beval(*b, st, default) {
                    match ceval_step(fuel1, *body, st, default) {
                        Option::None => Option::<Store>::None,
                        Option::Some(st1) => ceval_step(fuel1, Com::While { b: Box::new(*b), body: Box::new(*body) }, st1, default),
                    }
                } else {
                    Option::Some(st)
                }
            }
        }
    }
}


pub proof fn lemma_ceval_step_skip(fuel: nat, st: Store, default: int)
    requires fuel > 0
    ensures ceval_step(fuel, Com::Skip, st, default) == Option::Some(st)
{
    reveal_with_fuel(ceval_step, 1);
}

} // verus!