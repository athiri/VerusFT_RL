use vstd::prelude::*;

verus! {

pub open spec fn pe_lookup(st: PEState, x: Var) -> Option<int>
    decreases st.len()
{
    if st.len() == 0 {
        Option::None
    } else {
        let (v, val) = st[0];
        if v == x {
            Option::Some(val)
        } else {
            pe_lookup(st.skip(1), x)
        }
    }
}


pub open spec fn pe_aexp(st: PEState, a: AExp) -> PEAExp
    decreases a
{
    match a {
        AExp::ANum { n } => PEAExp::Static { n },
        AExp::AId { x } => {
            match pe_lookup(st, x) {
                Option::Some(v) => PEAExp::Static { n: v },
                Option::None => PEAExp::Dynamic { a },
            }
        }
        AExp::APlus { a1, a2 } => {
            match (pe_aexp(st, *a1), pe_aexp(st, *a2)) {
                (PEAExp::Static { n: n1 }, PEAExp::Static { n: n2 }) =>
                    PEAExp::Static { n: n1 + n2 },
                (PEAExp::Static { n: n1 }, PEAExp::Dynamic { a: a2 }) =>
                    PEAExp::Dynamic { a: AExp::APlus { a1: Box::new(AExp::ANum { n: n1 }), a2: Box::new(a2) } },
                (PEAExp::Dynamic { a: a1 }, PEAExp::Static { n: n2 }) =>
                    PEAExp::Dynamic { a: AExp::APlus { a1: Box::new(a1), a2: Box::new(AExp::ANum { n: n2 }) } },
                (PEAExp::Dynamic { a: a1 }, PEAExp::Dynamic { a: a2 }) =>
                    PEAExp::Dynamic { a: AExp::APlus { a1: Box::new(a1), a2: Box::new(a2) } },
            }
        }
        AExp::AMinus { a1, a2 } => {
            match (pe_aexp(st, *a1), pe_aexp(st, *a2)) {
                (PEAExp::Static { n: n1 }, PEAExp::Static { n: n2 }) =>
                    PEAExp::Static { n: n1 - n2 },
                _ => PEAExp::Dynamic { a },  // Simplified
            }
        }
        AExp::AMult { a1, a2 } => {
            match (pe_aexp(st, *a1), pe_aexp(st, *a2)) {
                (PEAExp::Static { n: n1 }, PEAExp::Static { n: n2 }) =>
                    PEAExp::Static { n: n1 * n2 },
                _ => PEAExp::Dynamic { a },  // Simplified
            }
        }
    }
}


pub enum PEAExp {
    Static { n: int },           // Fully evaluated constant
    Dynamic { a: AExp },         // Residual expression
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
    BEq { a1: AExp, a2: AExp },
    BLe { a1: AExp, a2: AExp },
    BNot { b: Box<BExp> },
    BAnd { b1: Box<BExp>, b2: Box<BExp> },
}

pub enum PEBExp {
    Static { b: bool },
    Dynamic { b: BExp },
}

pub type PEState = Seq<(Var, int)>;


pub open spec fn pe_bexp(st: PEState, b: BExp) -> PEBExp
    decreases b
{
    match b {
        BExp::BTrue => PEBExp::Static { b: true },
        BExp::BFalse => PEBExp::Static { b: false },
        BExp::BEq { a1, a2 } => {
            match (pe_aexp(st, a1), pe_aexp(st, a2)) {
                (PEAExp::Static { n: n1 }, PEAExp::Static { n: n2 }) =>
                    PEBExp::Static { b: n1 == n2 },
                _ => PEBExp::Dynamic { b },
            }
        }
        BExp::BLe { a1, a2 } => {
            match (pe_aexp(st, a1), pe_aexp(st, a2)) {
                (PEAExp::Static { n: n1 }, PEAExp::Static { n: n2 }) =>
                    PEBExp::Static { b: n1 <= n2 },
                _ => PEBExp::Dynamic { b },
            }
        }
        BExp::BNot { b: inner } => {
            match pe_bexp(st, *inner) {
                PEBExp::Static { b: v } => PEBExp::Static { b: !v },
                PEBExp::Dynamic { b: b_dyn } =>
                    PEBExp::Dynamic { b: BExp::BNot { b: Box::new(b_dyn) } },
            }
        }
        BExp::BAnd { b1, b2 } => {
            match pe_bexp(st, *b1) {
                PEBExp::Static { b: false } => PEBExp::Static { b: false },
                PEBExp::Static { b: true } => pe_bexp(st, *b2),
                PEBExp::Dynamic { b: b1_dyn } => PEBExp::Dynamic { b },  // Simplified
            }
        }
    }
}

} // verus!