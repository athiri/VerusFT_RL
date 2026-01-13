use vstd::prelude::*;

verus! {

pub enum Tm {
    // Booleans
    Tru,
    Fls,
    Ite { t1: Box<Tm>, t2: Box<Tm>, t3: Box<Tm> },  // if-then-else

    // Natural numbers
    Zro,                    // zero
    Scc { t: Box<Tm> },     // successor
    Prd { t: Box<Tm> },     // predecessor
    IsZro { t: Box<Tm> },   // iszero test
}

pub enum Ty {
    TBool,
    TNat,
}


pub open spec fn has_type(t: Tm, ty: Ty) -> bool
    decreases t
{
    match t {
        // T_True
        Tm::Tru => ty == Ty::TBool,
        // T_False
        Tm::Fls => ty == Ty::TBool,
        // T_If
        Tm::Ite { t1: cond, t2: then_br, t3: else_br } =>
            has_type(*cond, Ty::TBool) &&
            has_type(*then_br, ty) &&
            has_type(*else_br, ty),
        // T_Zero
        Tm::Zro => ty == Ty::TNat,
        // T_Succ
        Tm::Scc { t } => ty == Ty::TNat && has_type(*t, Ty::TNat),
        // T_Pred
        Tm::Prd { t } => ty == Ty::TNat && has_type(*t, Ty::TNat),
        // T_IsZero
        Tm::IsZro { t } => ty == Ty::TBool && has_type(*t, Ty::TNat),
    }
}

} // verus!