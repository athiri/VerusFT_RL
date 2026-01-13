use vstd::prelude::*;

verus! {

pub open spec fn empty_state() -> State {
    State {
        env: Map::<Id, Value>::empty(),
        next_id: 0,
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

pub type Env = Map<Id, Value>;

pub enum Value {
    VBool { b: bool },
    VNat { n: nat },
    VClosure { x: Id, ty: Ty, body: Box<Expr>, env: Env },
}

pub struct State {
    pub env: Env,
    pub next_id: Id,
}

pub open spec fn state_contains(st: State, x: Id) -> bool {
    st.env.dom().contains(x)
}


pub proof fn empty_state_is_empty()
    ensures forall|x: Id| !state_contains(empty_state(), x)
{
    assert forall|x: Id| !state_contains(empty_state(), x) by {
        assert(!Map::<Id, Value>::empty().dom().contains(x));
    }
}

} // verus!