use vstd::prelude::*;

verus! {

pub open spec fn store_ty_lookup(st_ty: StoreTyping, l: Loc) -> Option<Ty> {
    if l < st_ty.len() {
        Option::Some(st_ty[l as int])
    } else {
        Option::None
    }
}


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

pub type StoreTyping = Seq<Ty>;


pub open spec fn has_type(st_ty: StoreTyping, t: Tm, ty: Ty) -> bool
    decreases t
{
    match t {
        // T_Var (simplified - no context for this example)
        Tm::Var { x } => false,  // Variables require context

        // T_Abs
        Tm::Abs { x, ty: ty_param, body } => {
            match ty {
                Ty::TArrow { t1, t2 } => *t1 == ty_param,
                    // Simplified: just check parameter type matches
                _ => false,
            }
        }

        // T_App
        Tm::App { t1, t2 } => false,  // Simplified

        // T_Nat
        Tm::Nat { n } => ty == Ty::TNat,

        // T_Succ
        Tm::Scc { t } => ty == Ty::TNat && has_type(st_ty, *t, Ty::TNat),

        // T_Pred
        Tm::Prd { t } => ty == Ty::TNat && has_type(st_ty, *t, Ty::TNat),

        // T_Unit
        Tm::Unit => ty == Ty::TUnit,

        // T_Loc: A location has type Ref T if the store typing says so
        Tm::Loc { l } => {
            match ty {
                Ty::TRef { t: inner_ty } => {
                    match store_ty_lookup(st_ty, l) {
                        Option::Some(ty_l) => ty_l == *inner_ty,
                        Option::None => false,
                    }
                }
                _ => false,
            }
        }

        // T_Ref: ref t has type Ref T if t has type T
        Tm::Ref { t } => {
            match ty {
                Ty::TRef { t: inner_ty } => has_type(st_ty, *t, *inner_ty),
                _ => false,
            }
        }

        // T_Deref: !t has type T if t has type Ref T
        Tm::Deref { t } => {
            has_type(st_ty, *t, Ty::TRef { t: Box::new(ty) })
        }

        // T_Assign: t1 := t2 has type Unit
        Tm::Assign { t1, t2 } => {
            ty == Ty::TUnit &&
            exists|inner_ty: Ty| #![auto]
                has_type(st_ty, *t1, Ty::TRef { t: Box::new(inner_ty) }) &&
                has_type(st_ty, *t2, inner_ty)
        }
    }
}

} // verus!