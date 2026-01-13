use vstd::prelude::*;

verus! {

pub type Var = nat;

pub enum Ty {
    TUnit,                              // Unit type
    TBool,                              // Booleans
    TNat,                               // Natural numbers
    TArrow { t1: Box<Ty>, t2: Box<Ty> },  // Functions
    TProd { t1: Box<Ty>, t2: Box<Ty> },   // Products (pairs)
    TSum { t1: Box<Ty>, t2: Box<Ty> },    // Sums (either)
    TList { t: Box<Ty> },                 // Lists
}

pub struct Pair<A, B> {
    pub fst: A,
    pub snd: B,
}

pub enum Tm {
    // Basic lambda calculus
    Var { x: Var },
    Abs { x: Var, ty: Ty, body: Box<Tm> },
    App { t1: Box<Tm>, t2: Box<Tm> },

    // Unit
    Unit,

    // Booleans
    Tru,
    Fls,
    Ite { cond: Box<Tm>, then_br: Box<Tm>, else_br: Box<Tm> },

    // Natural numbers
    Nat { n: nat },
    Scc { t: Box<Tm> },
    Prd { t: Box<Tm> },
    IsZro { t: Box<Tm> },

    // Let binding
    Let { x: Var, t1: Box<Tm>, t2: Box<Tm> },  // let x = t1 in t2

    // Pairs (products)
    Pair { t1: Box<Tm>, t2: Box<Tm> },  // (t1, t2)
    Fst { t: Box<Tm> },                  // t.fst
    Snd { t: Box<Tm> },                  // t.snd

    // Sums
    Inl { t: Box<Tm>, ty: Ty },          // inl t as T
    Inr { t: Box<Tm>, ty: Ty },          // inr t as T
    Case { t: Box<Tm>, x1: Var, t1: Box<Tm>, x2: Var, t2: Box<Tm> },

    // Lists
    Nil { ty: Ty },                       // nil T
    Cons { t1: Box<Tm>, t2: Box<Tm> },   // cons t1 t2
    LCase { t: Box<Tm>, t1: Box<Tm>, x1: Var, x2: Var, t2: Box<Tm> },

    // Fixed point (recursion)
    Fix { t: Box<Tm> },
}


pub open spec fn has_type_simple(t: Tm, ty: Ty) -> bool
    decreases t
{
    match t {
        // Unit
        Tm::Unit => ty == Ty::TUnit,

        // Booleans
        Tm::Tru => ty == Ty::TBool,
        Tm::Fls => ty == Ty::TBool,

        // Naturals
        Tm::Nat { .. } => ty == Ty::TNat,
        Tm::Scc { t } => ty == Ty::TNat && has_type_simple(*t, Ty::TNat),
        Tm::Prd { t } => ty == Ty::TNat && has_type_simple(*t, Ty::TNat),
        Tm::IsZro { t } => ty == Ty::TBool && has_type_simple(*t, Ty::TNat),

        // Pairs: (t1, t2) : T1 * T2
        Tm::Pair { t1, t2 } => {
            match ty {
                Ty::TProd { t1: ty1, t2: ty2 } =>
                    has_type_simple(*t1, *ty1) && has_type_simple(*t2, *ty2),
                _ => false,
            }
        }

        // Fst: t.fst : T1 if t : T1 * T2
        Tm::Fst { t } => {
            exists|ty2: Ty| #![auto]
                has_type_simple(*t, Ty::TProd { t1: Box::new(ty), t2: Box::new(ty2) })
        }

        // Snd: t.snd : T2 if t : T1 * T2
        Tm::Snd { t } => {
            exists|ty1: Ty| #![auto]
                has_type_simple(*t, Ty::TProd { t1: Box::new(ty1), t2: Box::new(ty) })
        }

        // Inl: inl t as T1+T2 : T1+T2 if t : T1
        Tm::Inl { t, ty: ann_ty } => {
            ty == ann_ty &&
            match ann_ty {
                Ty::TSum { t1, t2: _ } => has_type_simple(*t, *t1),
                _ => false,
            }
        }

        // Inr: inr t as T1+T2 : T1+T2 if t : T2
        Tm::Inr { t, ty: ann_ty } => {
            ty == ann_ty &&
            match ann_ty {
                Ty::TSum { t1: _, t2 } => has_type_simple(*t, *t2),
                _ => false,
            }
        }

        // Nil: nil T : List T
        Tm::Nil { ty: elem_ty } => ty == (Ty::TList { t: Box::new(elem_ty) }),

        _ => false,  // Other cases need context
    }
}

} // verus!