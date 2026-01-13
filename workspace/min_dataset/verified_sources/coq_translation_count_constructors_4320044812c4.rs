use vstd::prelude::*;

verus! {

pub enum Ty {
    TBool,
    TNat,
    TUnit,
    TProd { t1: Box<Ty>, t2: Box<Ty> },  // Product type
    TSum { t1: Box<Ty>, t2: Box<Ty> },   // Sum type
    TArrow { t1: Box<Ty>, t2: Box<Ty> }, // Function type
}


pub open spec fn count_constructors(t: Ty) -> nat
    decreases t
{
    match t {
        Ty::TBool => 1,
        Ty::TNat => 1,
        Ty::TUnit => 1,
        Ty::TProd { t1, t2 } => 1 + count_constructors(*t1) + count_constructors(*t2),
        Ty::TSum { t1, t2 } => 1 + count_constructors(*t1) + count_constructors(*t2),
        Ty::TArrow { t1, t2 } => 1 + count_constructors(*t1) + count_constructors(*t2),
    }
}

} // verus!