use vstd::prelude::*;

verus! {

pub open spec fn in_scope(vs: VarSet, x: Var) -> bool {
    vs.contains(x)
}

pub open spec fn add_var(vs: VarSet, x: Var) -> VarSet {
    vs.insert(x)
}


pub type Var = nat;

pub enum Ty {
    TBool,
    TNat,
    TArrow { t1: Box<Ty>, t2: Box<Ty> },
}

pub enum Expr {
    Var { x: Var },
    Lam { x: Var, ty: Ty, body: Box<Expr> },
    App { e1: Box<Expr>, e2: Box<Expr> },
    Tru,
    Fls,
    If { cond: Box<Expr>, then_br: Box<Expr>, else_br: Box<Expr> },
    Zero,
    Succ { e: Box<Expr> },
    Pred { e: Box<Expr> },
    IsZero { e: Box<Expr> },
    Let { x: Var, def: Box<Expr>, body: Box<Expr> },
}

pub type VarSet = Set<Var>;


pub open spec fn is_well_formed(e: Expr, scope: VarSet) -> bool
    decreases e
{
    match e {
        Expr::Var { x } => in_scope(scope, x),
        Expr::Lam { x, body, .. } => is_well_formed(*body, add_var(scope, x)),
        Expr::App { e1, e2 } => is_well_formed(*e1, scope) && is_well_formed(*e2, scope),
        Expr::Tru => true,
        Expr::Fls => true,
        Expr::If { cond, then_br, else_br } =>
            is_well_formed(*cond, scope) &&
            is_well_formed(*then_br, scope) &&
            is_well_formed(*else_br, scope),
        Expr::Zero => true,
        Expr::Succ { e } => is_well_formed(*e, scope),
        Expr::Pred { e } => is_well_formed(*e, scope),
        Expr::IsZero { e } => is_well_formed(*e, scope),
        Expr::Let { x, def, body } =>
            is_well_formed(*def, scope) && is_well_formed(*body, add_var(scope, x)),
    }
}

} // verus!