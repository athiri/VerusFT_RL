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

pub open spec fn state_fresh_id(st: State) -> (Id, State) {
    let id = st.next_id;
    let new_state = State {
        env: st.env,
        next_id: id + 1,
    };
    (id, new_state)
}


pub proof fn state_fresh_id_increments(st: State)
    ensures
        state_fresh_id(st).0 == st.next_id,
        state_fresh_id(st).1.next_id == st.next_id + 1,
{
}

} // verus!