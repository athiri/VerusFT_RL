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

pub open spec fn env_lookup(env: Env, x: Id) -> Option<Value> {
    if env.dom().contains(x) {
        Option::Some(env[x])
    } else {
        Option::None
    }
}

pub open spec fn env_extend(env: Env, x: Id, v: Value) -> Env {
    env.insert(x, v)
}


pub proof fn env_extend_adds(env: Env, x: Id, v: Value)
    ensures env_lookup(env_extend(env, x, v), x) == Option::Some(v)
{
    assert(env.insert(x, v).dom().contains(x));
    assert(env.insert(x, v)[x] == v);
}

} // verus!