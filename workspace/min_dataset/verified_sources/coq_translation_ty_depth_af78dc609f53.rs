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


pub open spec fn ty_depth(t: Ty) -> nat
    decreases t
{
    match t {
        Ty::TBool => 0,
        Ty::TNat => 0,
        Ty::TUnit => 0,
        Ty::TProd { t1, t2 } => {
            let d1 = ty_depth(*t1);
            let d2 = ty_depth(*t2);
            1 + if d1 > d2 { d1 } else { d2 }
        }
        Ty::TSum { t1, t2 } => {
            let d1 = ty_depth(*t1);
            let d2 = ty_depth(*t2);
            1 + if d1 > d2 { d1 } else { d2 }
        }
        Ty::TArrow { t1, t2 } => {
            let d1 = ty_depth(*t1);
            let d2 = ty_depth(*t2);
            1 + if d1 > d2 { d1 } else { d2 }
        }
    }
}

} // verus!