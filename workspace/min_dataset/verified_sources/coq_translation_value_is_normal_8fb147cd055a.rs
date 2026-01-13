use vstd::prelude::*;

verus! {

pub enum Tm {
    C { n: int },              // Constant
    P { t1: Box<Tm>, t2: Box<Tm> },  // Plus
}

pub open spec fn can_reduce_pcc(t: Tm) -> bool {
    match t {
        Tm::P { t1, t2 } => {
            match (*t1, *t2) {
                (Tm::C { .. }, Tm::C { .. }) => true,
                _ => false,
            }
        }
        _ => false,
    }
}

pub open spec fn value(t: Tm) -> bool {
    match t {
        Tm::C { .. } => true,
        Tm::P { .. } => false,
    }
}


pub proof fn value_is_normal(t: Tm)
    requires value(t)
    ensures !can_reduce_pcc(t)
{
    match t {
        Tm::C { .. } => {}
        Tm::P { .. } => {}  // Not a value, so precondition is false
    }
}

} // verus!