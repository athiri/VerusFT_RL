use vstd::prelude::*;

verus! {

pub enum Ty {
    TBool,                                    // Boolean type
    TNat,                                     // Natural number type
    TArrow { t1: Box<Ty>, t2: Box<Ty> },     // Function type: t1 -> t2
}


pub open spec fn ty_size(ty: Ty) -> nat
    decreases ty
{
    match ty {
        Ty::TBool => 1,
        Ty::TNat => 1,
        Ty::TArrow { t1, t2 } => 1 + ty_size(*t1) + ty_size(*t2),
    }
}

} // verus!