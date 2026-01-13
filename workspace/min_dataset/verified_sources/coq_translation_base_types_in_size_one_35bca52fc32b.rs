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

pub open spec fn types_of_size(n: nat) -> Set<Ty> {
    Set::new(|t: Ty| ty_size(t) <= n)
}


pub enum Ty {
    TBool,
    TNat,
    TUnit,
    TProd { t1: Box<Ty>, t2: Box<Ty> },  // Product type
    TSum { t1: Box<Ty>, t2: Box<Ty> },   // Sum type
    TArrow { t1: Box<Ty>, t2: Box<Ty> }, // Function type
}


pub proof fn base_types_in_size_one()
    ensures
        types_of_size(1).contains(Ty::TBool),
        types_of_size(1).contains(Ty::TNat),
        types_of_size(1).contains(Ty::TUnit),
{
    assert(ty_size(Ty::TBool) == 1);
    assert(ty_size(Ty::TNat) == 1);
    assert(ty_size(Ty::TUnit) == 1);
}

} // verus!