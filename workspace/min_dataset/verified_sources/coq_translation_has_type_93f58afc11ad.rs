use vstd::prelude::*;

verus! {

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

pub open spec fn is_record_ty(ty: Ty) -> bool
    decreases ty
{
    match ty {
        Ty::TRNil => true,
        Ty::TRCons { label: _, ty: _, rest } => is_record_ty(*rest),
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


pub open spec fn has_type(t: Tm, ty: Ty) -> bool
    decreases t
{
    match t {
        // T_True, T_False
        Tm::Tru => ty == Ty::TBool,
        Tm::Fls => ty == Ty::TBool,

        // T_Nat
        Tm::Nat { .. } => ty == Ty::TNat,

        // T_RNil: {} : {}
        Tm::RNil => ty == Ty::TRNil,

        // T_RCons: {l=t, ...rest} : {l:T, ...Rest} if t:T and rest:Rest
        Tm::RCons { label, t, rest } => {
            match ty {
                Ty::TRCons { label: l, ty: ty_field, rest: ty_rest } =>
                    l == label &&
                    has_type(*t, *ty_field) &&
                    has_type(*rest, *ty_rest) &&
                    is_record_ty(*ty_rest),
                _ => false,
            }
        }

        // T_Proj: t.label : T if t : {..., label:T, ...}
        Tm::RProj { t, label } => {
            exists|rec_ty: Ty| #![auto]
                has_type(*t, rec_ty) &&
                is_record_ty(rec_ty) &&
                ty_lookup(label, rec_ty) == Option::Some(ty)
        }

        _ => false,  // Other terms need context
    }
}

} // verus!