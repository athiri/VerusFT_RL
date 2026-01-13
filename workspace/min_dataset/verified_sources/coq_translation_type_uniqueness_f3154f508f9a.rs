use vstd::prelude::*;

verus! {

pub open spec fn ctx_lookup(ctx: Context, x: Id) -> Option<Ty> {
    if ctx.dom().contains(x) {
        Option::Some(ctx[x])
    } else {
        Option::None
    }
}

pub open spec fn arrow_type(t1: Ty, t2: Ty) -> Ty {
    Ty::TArrow { t1: Box::new(t1), t2: Box::new(t2) }
}

pub open spec fn ctx_extend(ctx: Context, x: Id, ty: Ty) -> Context {
    ctx.insert(x, ty)
}


pub type Var = nat;

pub type Id = nat;

pub enum Ty {
    TBool,
    TNat,
    TArrow { t1: Box<Ty>, t2: Box<Ty> },
}

pub type Context = Map<Id, Ty>;

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

pub open spec fn infer_type(ctx: Context, e: Expr) -> Option<Ty>
    decreases e
{
    match e {
        Expr::Var { x } => ctx_lookup(ctx, x),

        Expr::BoolConst { .. } => Option::Some(Ty::TBool),

        Expr::NatConst { .. } => Option::Some(Ty::TNat),

        Expr::Plus { e1, e2 } => {
            match (infer_type(ctx, *e1), infer_type(ctx, *e2)) {
                (Option::Some(Ty::TNat), Option::Some(Ty::TNat)) =>
                    Option::Some(Ty::TNat),
                _ => Option::None,
            }
        }

        Expr::If { cond, then_br, else_br } => {
            match (infer_type(ctx, *cond), infer_type(ctx, *then_br), infer_type(ctx, *else_br)) {
                (Option::Some(Ty::TBool), Option::Some(ty1), Option::Some(ty2)) =>
                    if ty1 == ty2 { Option::Some(ty1) } else { Option::None },
                _ => Option::None,
            }
        }

        Expr::Lam { x, ty: ty_param, body } => {
            match infer_type(ctx_extend(ctx, x, ty_param), *body) {
                Option::Some(ty_body) =>
                    Option::Some(arrow_type(ty_param, ty_body)),
                Option::None => Option::None,
            }
        }

        Expr::App { e1, e2 } => {
            match (infer_type(ctx, *e1), infer_type(ctx, *e2)) {
                (Option::Some(Ty::TArrow { t1, t2 }), Option::Some(ty_arg)) =>
                    if *t1 == ty_arg { Option::Some(*t2) } else { Option::None },
                _ => Option::None,
            }
        }

        Expr::Eq { e1, e2 } => {
            match (infer_type(ctx, *e1), infer_type(ctx, *e2)) {
                (Option::Some(Ty::TNat), Option::Some(Ty::TNat)) =>
                    Option::Some(Ty::TBool),
                _ => Option::None,
            }
        }

        Expr::Lt { e1, e2 } => {
            match (infer_type(ctx, *e1), infer_type(ctx, *e2)) {
                (Option::Some(Ty::TNat), Option::Some(Ty::TNat)) =>
                    Option::Some(Ty::TBool),
                _ => Option::None,
            }
        }
    }
}


pub proof fn type_uniqueness(ctx: Context, e: Expr, ty1: Ty, ty2: Ty)
    requires
        infer_type(ctx, e) == Option::Some(ty1),
        infer_type(ctx, e) == Option::Some(ty2),
    ensures ty1 == ty2
{
}

} // verus!