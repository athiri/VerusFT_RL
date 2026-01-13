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


pub type Var = nat;

pub enum AExp {
    ANum { n: int },
    AId { x: Var },
    APlus { a1: Box<AExp>, a2: Box<AExp> },
    AMinus { a1: Box<AExp>, a2: Box<AExp> },
    AMult { a1: Box<AExp>, a2: Box<AExp> },
}

pub type PEState = Seq<(Var, int)>;

pub enum PEAExp {
    Static { n: int },           // Fully evaluated constant
    Dynamic { a: AExp },         // Residual expression
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

} // verus!