use vstd::prelude::*;

verus! {

pub open spec fn arrow_type(t1: Ty, t2: Ty) -> Ty {
    Ty::TArrow { t1: Box::new(t1), t2: Box::new(t2) }
}

pub open spec fn ctx_extend(ctx: Context, x: Id, ty: Ty) -> Context {
    ctx.insert(x, ty)
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

pub open spec fn var_expr(x: Id) -> Expr { Expr::Var { x } }

pub type Context = Map<Id, Ty>;

pub open spec fn has_type(ctx: Context, e: Expr, ty: Ty) -> bool
    decreases e
{
    match e {
        // T-Var: Variables have their type from the context
        Expr::Var { x } => {
            ctx.dom().contains(x) && ctx[x] == ty
        }

        // T-BoolConst: Boolean constants have type Bool
        Expr::BoolConst { .. } => {
            ty == Ty::TBool
        }

        // T-NatConst: Natural constants have type Nat
        Expr::NatConst { .. } => {
            ty == Ty::TNat
        }

        // T-Plus: Both operands must be Nat, result is Nat
        Expr::Plus { e1, e2 } => {
            ty == Ty::TNat &&
            has_type(ctx, *e1, Ty::TNat) &&
            has_type(ctx, *e2, Ty::TNat)
        }

        // T-If: Condition is Bool, branches have same type
        Expr::If { cond, then_br, else_br } => {
            has_type(ctx, *cond, Ty::TBool) &&
            has_type(ctx, *then_br, ty) &&
            has_type(ctx, *else_br, ty)
        }

        // T-Lam: Lambda abstraction
        Expr::Lam { x, ty: ty_param, body } => {
            match ty {
                Ty::TArrow { t1, t2 } =>
                    *t1 == ty_param &&
                    has_type(ctx_extend(ctx, x, ty_param), *body, *t2),
                _ => false,
            }
        }

        // T-App: Function application
        Expr::App { e1, e2 } => {
            exists|ty_arg: Ty|
                has_type(ctx, *e1, arrow_type(ty_arg, ty)) &&
                has_type(ctx, *e2, ty_arg)
        }

        // T-Eq: Equality comparison on nats produces bool
        Expr::Eq { e1, e2 } => {
            ty == Ty::TBool &&
            has_type(ctx, *e1, Ty::TNat) &&
            has_type(ctx, *e2, Ty::TNat)
        }

        // T-Lt: Less-than comparison on nats produces bool
        Expr::Lt { e1, e2 } => {
            ty == Ty::TBool &&
            has_type(ctx, *e1, Ty::TNat) &&
            has_type(ctx, *e2, Ty::TNat)
        }
    }
}


pub proof fn inference_implies_has_type_var(ctx: Context, x: Id, ty: Ty)
    requires ctx.dom().contains(x) && ctx[x] == ty
    ensures has_type(ctx, var_expr(x), ty)
{
}

} // verus!