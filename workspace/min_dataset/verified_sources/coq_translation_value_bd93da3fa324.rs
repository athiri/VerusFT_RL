use vstd::prelude::*;

verus! {

pub open spec fn is_record_value(t: Tm) -> bool
    decreases t
{
    match t {
        Tm::RNil => true,
        Tm::RCons { label: _, t, rest } => value(*t) && is_record_value(*rest),
        _ => false,
    }
}


pub type Var = nat;

pub type Label = nat;

pub enum Ty {
    TBool,
    TNat,
    TArrow { t1: Box<Ty>, t2: Box<Ty> },
    // Records: list of (label, type) pairs
    TRNil,                                          // Empty record type {}
    TRCons { label: Label, ty: Box<Ty>, rest: Box<Ty> },  // {label: ty, ...rest}
}

pub enum Tm {
    // Basic terms
    Var { x: Var },
    Abs { x: Var, ty: Ty, body: Box<Tm> },
    App { t1: Box<Tm>, t2: Box<Tm> },
    Tru,
    Fls,
    Nat { n: nat },

    // Records
    RNil,                                                // Empty record {}
    RCons { label: Label, t: Box<Tm>, rest: Box<Tm> },  // {label = t, ...rest}
    RProj { t: Box<Tm>, label: Label },                  // t.label (field access)
}


pub open spec fn value(t: Tm) -> bool
    decreases t
{
    match t {
        Tm::Abs { .. } => true,
        Tm::Tru => true,
        Tm::Fls => true,
        Tm::Nat { .. } => true,
        Tm::RNil => true,
        Tm::RCons { label: _, t, rest } => value(*t) && is_record_value(*rest),
        _ => false,
    }
}

} // verus!