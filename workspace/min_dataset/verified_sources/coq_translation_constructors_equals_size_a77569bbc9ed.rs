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


pub enum Ty {
    TBool,
    TNat,
    TUnit,
    TProd { t1: Box<Ty>, t2: Box<Ty> },  // Product type
    TSum { t1: Box<Ty>, t2: Box<Ty> },   // Sum type
    TArrow { t1: Box<Ty>, t2: Box<Ty> }, // Function type
}


pub proof fn constructors_equals_size(t: Ty)
    ensures count_constructors(t) == ty_size(t)
    decreases t
{
    match t {
        Ty::TBool => {}
        Ty::TNat => {}
        Ty::TUnit => {}
        Ty::TProd { t1, t2 } => {
            constructors_equals_size(*t1);
            constructors_equals_size(*t2);
        }
        Ty::TSum { t1, t2 } => {
            constructors_equals_size(*t1);
            constructors_equals_size(*t2);
        }
        Ty::TArrow { t1, t2 } => {
            constructors_equals_size(*t1);
            constructors_equals_size(*t2);
        }
    }
}

} // verus!