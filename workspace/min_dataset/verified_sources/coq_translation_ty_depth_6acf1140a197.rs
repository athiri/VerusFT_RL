use vstd::prelude::*;

verus! {

pub enum Ty {
    TBool,                                    // Boolean type
    TNat,                                     // Natural number type
    TArrow { t1: Box<Ty>, t2: Box<Ty> },     // Function type: t1 -> t2
}


pub open spec fn ty_depth(ty: Ty) -> nat
    decreases ty
{
    match ty {
        Ty::TBool => 0,
        Ty::TNat => 0,
        Ty::TArrow { t1, t2 } => {
            let d1 = ty_depth(*t1);
            let d2 = ty_depth(*t2);
            1 + if d1 > d2 { d1 } else { d2 }
        }
    }
}

} // verus!