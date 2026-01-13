use vstd::prelude::*;

verus! {

pub open spec fn state_lookup(st: State, x: Id) -> Option<Value> {
    if st.env.dom().contains(x) {
        Option::Some(st.env[x])
    } else {
        Option::None
    }
}


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

pub enum Value {
    VBool { b: bool },
    VNat { n: nat },
    VClosure { x: Id, ty: Ty, body: Box<Expr>, env: Env },
}

pub type Env = Map<Id, Value>;

pub struct State {
    pub env: Env,
    pub next_id: Id,
}

pub open spec fn state_equiv(st1: State, st2: State) -> bool {
    forall|x: Id| state_lookup(st1, x) == state_lookup(st2, x)
}


pub proof fn state_equiv_refl(st: State)
    ensures state_equiv(st, st)
{
}

} // verus!