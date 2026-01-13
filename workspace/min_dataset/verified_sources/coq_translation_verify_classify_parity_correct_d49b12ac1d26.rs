use vstd::prelude::*;

verus! {

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

pub open spec fn classify_parity(x: nat) -> Label {
    if x % 2 == 0 {
        Label::Even
    } else {
        Label::Odd
    }
}

pub open spec fn label_eq(l1: Label, l2: Label) -> bool {
    match (l1, l2) {
        (Label::Small, Label::Small) => true,
        (Label::Medium, Label::Medium) => true,
        (Label::Large, Label::Large) => true,
        (Label::Zero, Label::Zero) => true,
        (Label::Positive, Label::Positive) => true,
        (Label::Negative, Label::Negative) => true,
        (Label::Even, Label::Even) => true,
        (Label::Odd, Label::Odd) => true,
        (Label::Custom(a), Label::Custom(b)) => a == b,
        _ => false,
    }
}


pub proof fn verify_classify_parity_correct(x: nat)
    ensures
        (x % 2 == 0 ==> label_eq(classify_parity(x), Label::Even)) &&
        (x % 2 == 1 ==> label_eq(classify_parity(x), Label::Odd))
{
}

} // verus!