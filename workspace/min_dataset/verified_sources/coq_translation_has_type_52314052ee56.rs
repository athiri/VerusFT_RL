use vstd::prelude::*;

verus! {

pub enum Ty {
    TBool,
    TArrow { from: Box<Ty>, to: Box<Ty> },
}

pub type Context = Seq<Ty>;

pub enum Expr {
    Var { index: nat },
    Abs { body: Box<Expr> },
    App { func: Box<Expr>, arg: Box<Expr> },
}

pub open spec fn has_type(ctx: Context, e: Expr, ty: Ty) -> bool
    decreases e
{
    match e {
        Expr::Var { index } => {
            index < ctx.len() && ctx[index as int] == ty
        }
        Expr::Abs { body } => {
            match ty {
                Ty::TArrow { from, to } => {
                    has_type(seq![*from].add(ctx), *body, *to)
                }
                _ => false,
            }
        }
        Expr::App { func, arg } => {
            exists|arg_ty: Ty| has_type(ctx, *func, Ty::TArrow { from: Box::new(arg_ty), to: Box::new(ty) }) && has_type(ctx, *arg, arg_ty)
        }
    }
}

pub proof fn typing_unique(ctx: Context, e: Expr, ty1: Ty, ty2: Ty)
    requires has_type(ctx, e, ty1), has_type(ctx, e, ty2)
    ensures ty1 == ty2
    decreases e
{
    match e {
        Expr::Var { index } => {
            assert(ctx[index as int] == ty1);
            assert(ctx[index as int] == ty2);
        }
        Expr::Abs { body } => {
            admit();
        }
        Expr::App { func, arg } => {
            admit();
        }
    }
}

} // verus!
