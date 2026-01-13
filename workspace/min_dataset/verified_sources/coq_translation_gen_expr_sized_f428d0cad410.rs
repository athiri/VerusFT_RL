use vstd::prelude::*;

verus! {

pub open spec fn gen_var(scope: VarSet) -> Set<Expr> {
    Set::new(|e: Expr|
        exists|x: Var| scope.contains(x) && e == Expr::Var { x }
    )
}

pub open spec fn gen_zero() -> Set<Expr> {
    Set::new(|e: Expr| e == Expr::Zero)
}

pub open spec fn gen_bool_lit() -> Set<Expr> {
    Set::new(|e: Expr| e == Expr::Tru || e == Expr::Fls)
}

pub open spec fn gen_pred(base: Set<Expr>) -> Set<Expr> {
    Set::new(|e: Expr|
        exists|inner: Expr| base.contains(inner) && e == Expr::Pred { e: Box::new(inner) }
    )
}

pub open spec fn gen_iszero(base: Set<Expr>) -> Set<Expr> {
    Set::new(|e: Expr|
        exists|inner: Expr| base.contains(inner) && e == Expr::IsZero { e: Box::new(inner) }
    )
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

pub open spec fn gen_atomic(scope: VarSet) -> Set<Expr> {
    gen_bool_lit().union(gen_zero()).union(gen_var(scope))
}

pub open spec fn gen_succ(base: Set<Expr>) -> Set<Expr> {
    Set::new(|e: Expr|
        exists|inner: Expr| base.contains(inner) && e == Expr::Succ { e: Box::new(inner) }
    )
}


pub open spec fn gen_expr_sized(scope: VarSet, size: nat) -> Set<Expr>
    decreases size
{
    if size == 0 {
        Set::empty()
    } else if size == 1 {
        gen_atomic(scope)
    } else {
        let smaller = gen_expr_sized(scope, (size - 1) as nat);
        let atoms = gen_atomic(scope);
        let succs = gen_succ(smaller);
        let preds = gen_pred(smaller);
        let iszeros = gen_iszero(smaller);
        atoms.union(succs).union(preds).union(iszeros)
    }
}

} // verus!