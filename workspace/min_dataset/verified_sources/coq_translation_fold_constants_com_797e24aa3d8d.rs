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

pub enum Com {
    CSkip,
    CAsgn { x: Var, a: Box<AExp> },
    CSeq { c1: Box<Com>, c2: Box<Com> },
    CIf { b: Box<BExp>, ct: Box<Com>, cf: Box<Com> },
    CWhile { b: Box<BExp>, body: Box<Com> },
}


pub open spec fn fold_constants_com(c: Com) -> Com
    decreases c
{
    match c {
        Com::CSkip => Com::CSkip,
        Com::CAsgn { x, a } => Com::CAsgn { x, a: Box::new(fold_constants_aexp(*a)) },
        Com::CSeq { c1, c2 } => Com::CSeq {
            c1: Box::new(fold_constants_com(*c1)),
            c2: Box::new(fold_constants_com(*c2)),
        },
        Com::CIf { b, ct, cf } => {
            let b_folded = fold_constants_bexp(*b);
            match b_folded {
                BExp::BTrue => fold_constants_com(*ct),
                BExp::BFalse => fold_constants_com(*cf),
                _ => Com::CIf {
                    b: Box::new(b_folded),
                    ct: Box::new(fold_constants_com(*ct)),
                    cf: Box::new(fold_constants_com(*cf)),
                },
            }
        }
        Com::CWhile { b, body } => {
            let b_folded = fold_constants_bexp(*b);
            match b_folded {
                BExp::BFalse => Com::CSkip,
                _ => Com::CWhile {
                    b: Box::new(b_folded),
                    body: Box::new(fold_constants_com(*body)),
                },
            }
        }
    }
}

} // verus!