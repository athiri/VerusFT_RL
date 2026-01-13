use vstd::prelude::*;

verus! {

pub enum Tm {
    C { n: int },              // Constant
    P { t1: Box<Tm>, t2: Box<Tm> },  // Plus
}


pub open spec fn tm_size(t: Tm) -> nat
    decreases t
{
    match t {
        Tm::C { .. } => 1,
        Tm::P { t1, t2 } => 1 + tm_size(*t1) + tm_size(*t2),
    }
}

} // verus!