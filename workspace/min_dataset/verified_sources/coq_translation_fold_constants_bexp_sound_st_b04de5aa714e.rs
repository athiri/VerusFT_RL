use vstd::prelude::*;

verus! {

pub open spec fn fold_constants_aexp(a: AExp) -> AExp
    decreases a
{
    match a {
        AExp::ANum { n } => AExp::ANum { n },
        AExp::AId { x } => AExp::AId { x },
        AExp::APlus { a1, a2 } => {
            let a1_folded = fold_constants_aexp(*a1);
            let a2_folded = fold_constants_aexp(*a2);
            match (a1_folded, a2_folded) {
                (AExp::ANum { n: n1 }, AExp::ANum { n: n2 }) => AExp::ANum { n: n1 + n2 },
                (a1f, a2f) => AExp::APlus { a1: Box::new(a1f), a2: Box::new(a2f) },
            }
        }
        AExp::AMinus { a1, a2 } => {
            let a1_folded = fold_constants_aexp(*a1);
            let a2_folded = fold_constants_aexp(*a2);
            match (a1_folded, a2_folded) {
                (AExp::ANum { n: n1 }, AExp::ANum { n: n2 }) => AExp::ANum { n: n1 - n2 },
                (a1f, a2f) => AExp::AMinus { a1: Box::new(a1f), a2: Box::new(a2f) },
            }
        }
        AExp::AMult { a1, a2 } => {
            let a1_folded = fold_constants_aexp(*a1);
            let a2_folded = fold_constants_aexp(*a2);
            match (a1_folded, a2_folded) {
                (AExp::ANum { n: n1 }, AExp::ANum { n: n2 }) => AExp::ANum { n: n1 * n2 },
                (a1f, a2f) => AExp::AMult { a1: Box::new(a1f), a2: Box::new(a2f) },
            }
        }
    }
}

pub open spec fn store_get(st: Store, x: Var) -> int {
    if st.dom().contains(x) { st[x] } else { 0 }
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

pub proof fn fold_constants_aexp_sound_st(a: AExp, st: Store)
    ensures aeval(a, st) == aeval(fold_constants_aexp(a), st)
    decreases a
{
    match a {
        AExp::ANum { n } => {}
        AExp::AId { x } => {}
        AExp::APlus { a1, a2 } => {
            fold_constants_aexp_sound_st(*a1, st);
            fold_constants_aexp_sound_st(*a2, st);
        }
        AExp::AMinus { a1, a2 } => {
            fold_constants_aexp_sound_st(*a1, st);
            fold_constants_aexp_sound_st(*a2, st);
        }
        AExp::AMult { a1, a2 } => {
            fold_constants_aexp_sound_st(*a1, st);
            fold_constants_aexp_sound_st(*a2, st);
        }
    }
}

pub open spec fn fold_constants_bexp(b: BExp) -> BExp
    decreases b
{
    match b {
        BExp::BTrue => BExp::BTrue,
        BExp::BFalse => BExp::BFalse,
        BExp::BEq { a1, a2 } => {
            let a1_folded = fold_constants_aexp(*a1);
            let a2_folded = fold_constants_aexp(*a2);
            match (a1_folded, a2_folded) {
                (AExp::ANum { n: n1 }, AExp::ANum { n: n2 }) => {
                    if n1 == n2 { BExp::BTrue } else { BExp::BFalse }
                }
                (a1f, a2f) => BExp::BEq { a1: Box::new(a1f), a2: Box::new(a2f) },
            }
        }
        BExp::BLe { a1, a2 } => {
            let a1_folded = fold_constants_aexp(*a1);
            let a2_folded = fold_constants_aexp(*a2);
            match (a1_folded, a2_folded) {
                (AExp::ANum { n: n1 }, AExp::ANum { n: n2 }) => {
                    if n1 <= n2 { BExp::BTrue } else { BExp::BFalse }
                }
                (a1f, a2f) => BExp::BLe { a1: Box::new(a1f), a2: Box::new(a2f) },
            }
        }
        BExp::BNot { b1 } => {
            let b1_folded = fold_constants_bexp(*b1);
            match b1_folded {
                BExp::BTrue => BExp::BFalse,
                BExp::BFalse => BExp::BTrue,
                _ => BExp::BNot { b1: Box::new(b1_folded) },
            }
        }
        BExp::BAnd { b1, b2 } => {
            let b1_folded = fold_constants_bexp(*b1);
            let b2_folded = fold_constants_bexp(*b2);
            match (b1_folded, b2_folded) {
                (BExp::BTrue, b2f) => b2f,
                (BExp::BFalse, _) => BExp::BFalse,
                (b1f, BExp::BTrue) => b1f,
                (_, BExp::BFalse) => BExp::BFalse,
                (b1f, b2f) => BExp::BAnd { b1: Box::new(b1f), b2: Box::new(b2f) },
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

pub type Store = Map<Var, int>;


pub proof fn fold_constants_bexp_sound_st(b: BExp, st: Store)
    ensures beval(b, st) == beval(fold_constants_bexp(b), st)
    decreases b
{
    match b {
        BExp::BTrue => {}
        BExp::BFalse => {}
        BExp::BEq { a1, a2 } => {
            fold_constants_aexp_sound_st(*a1, st);
            fold_constants_aexp_sound_st(*a2, st);
        }
        BExp::BLe { a1, a2 } => {
            fold_constants_aexp_sound_st(*a1, st);
            fold_constants_aexp_sound_st(*a2, st);
        }
        BExp::BNot { b1 } => {
            fold_constants_bexp_sound_st(*b1, st);
        }
        BExp::BAnd { b1, b2 } => {
            fold_constants_bexp_sound_st(*b1, st);
            fold_constants_bexp_sound_st(*b2, st);
        }
    }
}

} // verus!