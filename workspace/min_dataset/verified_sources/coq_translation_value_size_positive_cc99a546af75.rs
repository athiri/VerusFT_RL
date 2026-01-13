use vstd::prelude::*;

verus! {

pub type Id = nat;

pub type Var = nat;

pub enum Ty {
    TBool,
    TNat,
    TArrow { t1: Box<Ty>, t2: Box<Ty> },
}

pub enum Expr {
    Var { x: Id },
    BoolConst { b: bool },
    NatConst { n: nat },
    Plus { e1: Box<Expr>, e2: Box<Expr> },
    If { cond: Box<Expr>, then_br: Box<Expr>, else_br: Box<Expr> },
    App { e1: Box<Expr>, e2: Box<Expr> },
    Lam { x: Id, ty: Ty, body: Box<Expr> },
    Eq { e1: Box<Expr>, e2: Box<Expr> },
    Lt { e1: Box<Expr>, e2: Box<Expr> },
}

pub type Env = Map<Id, Value>;

pub enum Value {
    VBool { b: bool },                                        // Boolean value
    VNat { n: nat },                                          // Natural number value
    VClosure { x: Id, ty: Ty, body: Box<Expr>, env: Env },   // Closure
}

pub open spec fn value_size(v: Value) -> nat {
    match v {
        Value::VBool { .. } => 1,
        Value::VNat { .. } => 1,
        Value::VClosure { .. } => 2,  // Closures have fixed size for simplicity
    }
}


pub proof fn value_size_positive(v: Value)
    ensures value_size(v) >= 1
{
}

} // verus!