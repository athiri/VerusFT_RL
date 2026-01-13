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


pub open spec fn count_arrows(t: Ty) -> nat
    decreases t
{
    match t {
        Ty::TBool => 0,
        Ty::TNat => 0,
        Ty::TUnit => 0,
        Ty::TProd { t1, t2 } => count_arrows(*t1) + count_arrows(*t2),
        Ty::TSum { t1, t2 } => count_arrows(*t1) + count_arrows(*t2),
        Ty::TArrow { t1, t2 } => 1 + count_arrows(*t1) + count_arrows(*t2),
    }
}

} // verus!