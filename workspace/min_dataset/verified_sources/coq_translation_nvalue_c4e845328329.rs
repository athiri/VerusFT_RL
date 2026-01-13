use vstd::prelude::*;

verus! {

pub enum Tm {
    // Booleans
    Tru,
    Fls,
    Ite { t1: Box<Tm>, t2: Box<Tm>, t3: Box<Tm> },  // if-then-else

    // Natural numbers
    Zro,                    // zero
    Scc { t: Box<Tm> },     // successor
    Prd { t: Box<Tm> },     // predecessor
    IsZro { t: Box<Tm> },   // iszero test
}


pub open spec fn nvalue(t: Tm) -> bool
    decreases t
{
    match t {
        Tm::Zro => true,
        Tm::Scc { t } => nvalue(*t),
        _ => false,
    }
}

} // verus!