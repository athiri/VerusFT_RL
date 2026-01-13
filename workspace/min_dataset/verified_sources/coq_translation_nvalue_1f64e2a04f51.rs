use vstd::prelude::*;

verus! {

pub type Var = nat;

pub enum Ty {
    TUnit,                          // Unit type (result of assignment)
    TNat,                           // Natural numbers
    TArrow { t1: Box<Ty>, t2: Box<Ty> },  // Function type
    TRef { t: Box<Ty> },            // Reference type Ref T
}

pub type Loc = nat;

pub enum Tm {
    // Lambda calculus core
    Var { x: Var },
    Abs { x: Var, ty: Ty, body: Box<Tm> },
    App { t1: Box<Tm>, t2: Box<Tm> },

    // Natural numbers
    Nat { n: nat },
    Scc { t: Box<Tm> },
    Prd { t: Box<Tm> },

    // Unit
    Unit,

    // References
    Ref { t: Box<Tm> },             // ref t - allocation
    Deref { t: Box<Tm> },           // !t - dereference
    Assign { t1: Box<Tm>, t2: Box<Tm> },  // t1 := t2 - assignment
    Loc { l: Loc },                 // loc l - concrete location
}


pub open spec fn nvalue(t: Tm) -> bool
    decreases t
{
    match t {
        Tm::Nat { .. } => true,
        Tm::Scc { t } => nvalue(*t),
        _ => false,
    }
}

} // verus!