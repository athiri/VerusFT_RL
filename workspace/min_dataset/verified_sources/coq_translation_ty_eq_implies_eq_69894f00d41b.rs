use vstd::prelude::*;

verus! {

pub open spec fn ty_eq(t1: Ty, t2: Ty) -> bool
    decreases t1, t2
{
    match (t1, t2) {
        (Ty::TBool, Ty::TBool) => true,
        (Ty::TArrow { t1: a1, t2: a2 }, Ty::TArrow { t1: b1, t2: b2 }) =>
            ty_eq(*a1, *b1) && ty_eq(*a2, *b2),
        _ => false,
    }
}


pub enum Ty {
    TBool,
    TArrow { t1: Box<Ty>, t2: Box<Ty> },
}


pub proof fn ty_eq_implies_eq(t1: Ty, t2: Ty)
    requires ty_eq(t1, t2)
    ensures t1 == t2
    decreases t1, t2
{
    match (t1, t2) {
        (Ty::TBool, Ty::TBool) => {}
        (Ty::TArrow { t1: a1, t2: a2 }, Ty::TArrow { t1: b1, t2: b2 }) => {
            ty_eq_implies_eq(*a1, *b1);
            ty_eq_implies_eq(*a2, *b2);
        }
        _ => {}
    }
}

} // verus!