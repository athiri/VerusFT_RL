use vstd::prelude::*;

verus! {

pub open spec fn store_get(st: Store, x: Var) -> int {
    if st.dom().contains(x) { st[x] } else { 0 }
}


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


pub type Var = nat;

pub enum AExp {
    ANum { n: int },
    AId { x: Var },
    APlus { a1: Box<AExp>, a2: Box<AExp> },
    AMinus { a1: Box<AExp>, a2: Box<AExp> },
    AMult { a1: Box<AExp>, a2: Box<AExp> },
}

pub type Store = Map<Var, int>;


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

} // verus!