use vstd::prelude::*;

verus! {

pub open spec fn reduce_pcc(t: Tm) -> Tm {
    match t {
        Tm::P { t1, t2 } => {
            match (*t1, *t2) {
                (Tm::C { n: n1 }, Tm::C { n: n2 }) => Tm::C { n: n1 + n2 },
                _ => t,  // unreachable if can_reduce_pcc is true
            }
        }
        _ => t,
    }
}


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

pub open spec fn tm_size(t: Tm) -> nat
    decreases t
{
    match t {
        Tm::C { .. } => 1,
        Tm::P { t1, t2 } => 1 + tm_size(*t1) + tm_size(*t2),
    }
}


pub proof fn reduce_pcc_decreases_size(t: Tm)
    requires can_reduce_pcc(t)
    ensures tm_size(reduce_pcc(t)) < tm_size(t)
{
    reveal_with_fuel(tm_size, 3);
    match t {
        Tm::P { t1, t2 } => {
            match (*t1, *t2) {
                (Tm::C { n: _ }, Tm::C { n: _ }) => {
                    // P(C n1, C n2) reduces to C(n1+n2)
                    // Size goes from 1 + 1 + 1 = 3 to 1
                }
                _ => {}
            }
        }
        _ => {}
    }
}

} // verus!