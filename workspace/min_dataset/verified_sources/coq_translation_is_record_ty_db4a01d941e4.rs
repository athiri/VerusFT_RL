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


pub open spec fn is_record_ty(ty: Ty) -> bool
    decreases ty
{
    match ty {
        Ty::TRNil => true,
        Ty::TRCons { label: _, ty: _, rest } => is_record_ty(*rest),
        _ => false,
    }
}

} // verus!