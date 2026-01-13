use vstd::prelude::*;

verus! {

pub enum Tm {
    C { n: int },
    P { t1: Box<Tm>, t2: Box<Tm> },
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

pub open spec fn reduce_pcc(t: Tm) -> Tm {
    match t {
        Tm::P { t1, t2 } => {
            match (*t1, *t2) {
                (Tm::C { n: n1 }, Tm::C { n: n2 }) => Tm::C { n: n1 + n2 },
                _ => t,
            }
        }
        _ => t,
    }
}

pub proof fn example_can_reduce()
    ensures can_reduce_pcc(Tm::P { t1: Box::new(Tm::C { n: 1 }), t2: Box::new(Tm::C { n: 2 }) })
{}

pub proof fn example_reduce_1_plus_2()
    ensures (reduce_pcc(Tm::P { t1: Box::new(Tm::C { n: 1 }), t2: Box::new(Tm::C { n: 2 }) }) == (Tm::C { n: 3 }))
{}

} // verus!
