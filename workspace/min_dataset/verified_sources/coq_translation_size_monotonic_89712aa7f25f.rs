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


pub proof fn size_monotonic(n: nat, m: nat, t: Ty)
    requires n <= m, types_of_size(n).contains(t)
    ensures types_of_size(m).contains(t)
{
    assert(ty_size(t) <= n);
    assert(n <= m);
    assert(ty_size(t) <= m);
}

} // verus!