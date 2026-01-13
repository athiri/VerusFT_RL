use vstd::prelude::*;

verus! {

pub open spec fn ctx_extend(ctx: Context, x: Var, ty: Ty) -> Context {
    ctx.insert(x, ty)
}


pub type Var = nat;

pub enum Ty {
    TBool,
    TArrow { t1: Box<Ty>, t2: Box<Ty> },
}

pub enum Tm {
    Var { x: Var },
    Abs { x: Var, ty: Ty, body: Box<Tm> },
    App { t1: Box<Tm>, t2: Box<Tm> },
    Tru,
    Fls,
    Ite { cond: Box<Tm>, then_br: Box<Tm>, else_br: Box<Tm> },
}

pub type Context = Map<Var, Ty>;

pub open spec fn empty_ctx() -> Context {
    Map::<Var, Ty>::empty()
}

pub open spec fn has_type(ctx: Context, t: Tm, ty: Ty) -> bool
    decreases t
{
    match t {
        Tm::Var { x } => ctx.dom().contains(x) && ctx[x] == ty,
        Tm::Abs { x, ty: ty_param, body } => {
            match ty {
                Ty::TArrow { t1, t2 } =>
                    *t1 == ty_param &&
                    has_type(ctx_extend(ctx, x, ty_param), *body, *t2),
                _ => false,
            }
        }
        Tm::App { t1, t2 } => {
            exists|ty_arg: Ty| #![auto]
                has_type(ctx, *t1, Ty::TArrow { t1: Box::new(ty_arg), t2: Box::new(ty) }) &&
                has_type(ctx, *t2, ty_arg)
        }
        Tm::Tru => ty == Ty::TBool,
        Tm::Fls => ty == Ty::TBool,
        Tm::Ite { cond, then_br, else_br } =>
            has_type(ctx, *cond, Ty::TBool) &&
            has_type(ctx, *then_br, ty) &&
            has_type(ctx, *else_br, ty),
    }
}


pub proof fn example_true_typed()
    ensures has_type(empty_ctx(), Tm::Tru, Ty::TBool)
{
}

} // verus!