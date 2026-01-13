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


pub open spec fn eval(t: Tm, fuel: nat) -> Tm
    decreases fuel
{
    if fuel == 0 {
        t
    } else if can_reduce_pcc(t) {
        eval(reduce_pcc(t), (fuel - 1) as nat)
    } else {
        match t {
            Tm::P { t1, t2 } => {
                // Try to reduce left side first
                let t1_reduced = eval(*t1, (fuel - 1) as nat);
                if t1_reduced != *t1 {
                    Tm::P { t1: Box::new(t1_reduced), t2: t2 }
                } else {
                    // Then try right side
                    let t2_reduced = eval(*t2, (fuel - 1) as nat);
                    Tm::P { t1: t1, t2: Box::new(t2_reduced) }
                }
            }
            _ => t,
        }
    }
}

} // verus!