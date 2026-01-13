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

pub open spec fn state_extend(st: State, x: Id, v: Value) -> State {
    State {
        env: st.env.insert(x, v),
        next_id: st.next_id,
    }
}

pub open spec fn state_contains(st: State, x: Id) -> bool {
    st.env.dom().contains(x)
}


pub proof fn state_extend_contains(st: State, x: Id, v: Value)
    ensures state_contains(state_extend(st, x, v), x)
{
    assert(st.env.insert(x, v).dom().contains(x));
}

} // verus!