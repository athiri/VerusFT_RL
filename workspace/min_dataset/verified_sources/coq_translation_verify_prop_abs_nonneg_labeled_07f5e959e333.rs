use vstd::prelude::*;

verus! {

pub open spec fn labeled(label: Label, prop: bool) -> LabeledProp {
    LabeledProp { label, holds: prop }
}

pub open spec fn classify_sign(x: int) -> Label {
    if x == 0 {
        Label::Zero
    } else if x > 0 {
        Label::Positive
    } else {
        Label::Negative
    }
}


pub type Id = nat;

pub type Var = nat;

pub enum Expr {
    Var { x: Var },
    Tru,
    Fls,
    If { cond: Box<Expr>, then_br: Box<Expr>, else_br: Box<Expr> },
    Zero,
    Succ { e: Box<Expr> },
    Pred { e: Box<Expr> },
    IsZero { e: Box<Expr> },
    And { e1: Box<Expr>, e2: Box<Expr> },
    Or { e1: Box<Expr>, e2: Box<Expr> },
    Not { e: Box<Expr> },
    While { cond: Box<Expr>, body: Box<Expr> },
    Seq { e1: Box<Expr>, e2: Box<Expr> },
}

pub type Env = Map<Id, Value>;

pub enum Ty {
    TUnit,                              // Unit type
    TBool,                              // Booleans
    TNat,                               // Natural numbers
    TArrow { t1: Box<Ty>, t2: Box<Ty> },  // Functions
    TProd { t1: Box<Ty>, t2: Box<Ty> },   // Products (pairs)
    TSum { t1: Box<Ty>, t2: Box<Ty> },    // Sums (either)
    TList { t: Box<Ty> },                 // Lists
}

pub enum Value {
    VBool { b: bool },
    VNat { n: nat },
    VClosure { x: Id, ty: Ty, body: Box<Expr>, env: Env },
}

pub enum Label {
    Small,      // Value is small (< 10)
    Medium,     // Value is medium (10-100)
    Large,      // Value is large (> 100)
    Zero,       // Value is zero
    Positive,   // Value is positive
    Negative,   // Value is negative (for int)
    Even,       // Value is even
    Odd,        // Value is odd
    Custom(int), // Custom label with identifier
}

pub struct LabeledProp {
    pub label: Label,
    pub holds: bool,
}

pub open spec fn prop_abs_nonneg_labeled(x: int) -> LabeledProp {
    let abs_x = if x >= 0 { x } else { -x };
    labeled(classify_sign(x), abs_x >= 0)
}

pub open spec fn labeled_holds(lp: LabeledProp) -> bool {
    lp.holds
}


pub proof fn verify_prop_abs_nonneg_labeled(x: int)
    ensures labeled_holds(prop_abs_nonneg_labeled(x))
{
}

} // verus!