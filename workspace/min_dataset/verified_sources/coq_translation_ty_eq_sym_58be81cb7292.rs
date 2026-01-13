use vstd::prelude::*;

verus! {

pub enum Ty {
    TBool,                                    // Boolean type
    TNat,                                     // Natural number type
    TArrow { t1: Box<Ty>, t2: Box<Ty> },     // Function type: t1 -> t2
}

pub open spec fn ty_eq(ty1: Ty, ty2: Ty) -> bool {
    ty1 == ty2
}


pub proof fn ty_eq_sym(ty1: Ty, ty2: Ty)
    requires ty_eq(ty1, ty2)
    ensures ty_eq(ty2, ty1)
{
}

} // verus!