use vstd::prelude::*;

verus! {

pub open spec fn ctx_lookup(ctx: Context, x: Var) -> Option<Ty> {
    if ctx.dom().contains(x) {
        Option::Some(ctx[x])
    } else {
        Option::None
    }
}

pub open spec fn ty_eq(t1: Ty, t2: Ty) -> bool
    decreases t1, t2
{
    match (t1, t2) {
        (Ty::TBool, Ty::TBool) => true,
        (Ty::TNat, Ty::TNat) => true,
        (Ty::TArrow { t1: a1, t2: a2 }, Ty::TArrow { t1: b1, t2: b2 }) =>
            ty_eq(*a1, *b1) && ty_eq(*a2, *b2),
        _ => false,
    }
}

pub open spec fn ctx_extend(ctx: Context, x: Var, ty: Ty) -> Context {
    ctx.insert(x, ty)
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
}

pub type Context = Map<Var, Ty>;


pub open spec fn has_type(ctx: Context, e: Expr, ty: Ty) -> bool
    decreases e
{
    match e {
        Expr::Var { x } => ctx_lookup(ctx, x) == Option::Some(ty),

        Expr::Lam { x, ty: ty_param, body } => {
            match ty {
                Ty::TArrow { t1, t2 } =>
                    ty_eq(*t1, ty_param) &&
                    has_type(ctx_extend(ctx, x, ty_param), *body, *t2),
                _ => false,
            }
        }

        Expr::App { e1, e2 } => {
            exists|ty_arg: Ty|
                has_type(ctx, *e1, Ty::TArrow { t1: Box::new(ty_arg), t2: Box::new(ty) }) &&
                has_type(ctx, *e2, ty_arg)
        }

        Expr::Tru => ty == Ty::TBool,
        Expr::Fls => ty == Ty::TBool,

        Expr::If { cond, then_br, else_br } => {
            has_type(ctx, *cond, Ty::TBool) &&
            has_type(ctx, *then_br, ty) &&
            has_type(ctx, *else_br, ty)
        }

        Expr::Zero => ty == Ty::TNat,

        Expr::Succ { e } => ty == Ty::TNat && has_type(ctx, *e, Ty::TNat),

        Expr::Pred { e } => ty == Ty::TNat && has_type(ctx, *e, Ty::TNat),

        Expr::IsZero { e } => ty == Ty::TBool && has_type(ctx, *e, Ty::TNat),
    }
}

} // verus!