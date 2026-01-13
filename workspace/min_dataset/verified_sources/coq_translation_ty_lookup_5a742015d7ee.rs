use vstd::prelude::*;

verus! {

pub type Label = nat;

pub enum Ty {
    TBool,
    TNat,
    TArrow { t1: Box<Ty>, t2: Box<Ty> },
    // Records: list of (label, type) pairs
    TRNil,                                          // Empty record type {}
    TRCons { label: Label, ty: Box<Ty>, rest: Box<Ty> },  // {label: ty, ...rest}
}


pub open spec fn ty_lookup(label: Label, ty: Ty) -> Option<Ty>
    decreases ty
{
    match ty {
        Ty::TRNil => Option::None,
        Ty::TRCons { label: l, ty: t, rest } =>
            if l == label {
                Option::Some(*t)
            } else {
                ty_lookup(label, *rest)
            },
        _ => Option::None,
    }
}

} // verus!