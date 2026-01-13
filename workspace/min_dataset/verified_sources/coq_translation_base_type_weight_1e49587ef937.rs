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

pub open spec fn type_frequency(t: Ty) -> nat {
    match t {
        Ty::TBool => 10,   // Common
        Ty::TNat => 10,    // Common
        Ty::TUnit => 5,    // Less common
        Ty::TProd { .. } => 3,
        Ty::TSum { .. } => 2,
        Ty::TArrow { .. } => 5,
    }
}

pub open spec fn base_type_total_weight() -> nat {
    type_frequency(Ty::TBool) + type_frequency(Ty::TNat) + type_frequency(Ty::TUnit)
}


pub proof fn base_type_weight()
    ensures base_type_total_weight() == 25
{
    assert(type_frequency(Ty::TBool) == 10);
    assert(type_frequency(Ty::TNat) == 10);
    assert(type_frequency(Ty::TUnit) == 5);
}

} // verus!