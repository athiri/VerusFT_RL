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