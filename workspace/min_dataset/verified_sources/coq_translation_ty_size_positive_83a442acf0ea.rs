use vstd::prelude::*;

verus! {

pub open spec fn ty_size(t: Ty) -> nat
    decreases t
{
    match t {
        Ty::TBool => 1,
        Ty::TNat => 1,
        Ty::TUnit => 1,
        Ty::TProd { t1, t2 } => 1 + ty_size(*t1) + ty_size(*t2),
        Ty::TSum { t1, t2 } => 1 + ty_size(*t1) + ty_size(*t2),
        Ty::TArrow { t1, t2 } => 1 + ty_size(*t1) + ty_size(*t2),
    }
}


pub enum Ty {
    TBool,
    TNat,
    TUnit,
    TProd { t1: Box<Ty>, t2: Box<Ty> },  // Product type
    TSum { t1: Box<Ty>, t2: Box<Ty> },   // Sum type
    TArrow { t1: Box<Ty>, t2: Box<Ty> }, // Function type
}


pub proof fn ty_size_positive(t: Ty)
    ensures ty_size(t) >= 1
    decreases t
{
    match t {
        Ty::TBool => {}
        Ty::TNat => {}
        Ty::TUnit => {}
        Ty::TProd { t1, t2 } => {
            ty_size_positive(*t1);
            ty_size_positive(*t2);
        }
        Ty::TSum { t1, t2 } => {
            ty_size_positive(*t1);
            ty_size_positive(*t2);
        }
        Ty::TArrow { t1, t2 } => {
            ty_size_positive(*t1);
            ty_size_positive(*t2);
        }
    }
}

} // verus!