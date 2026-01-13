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
    TArrow { t1: Box<Ty>, t2: Box<Ty> },
}

pub enum Tm {
    Var { x: Var },
    Abs { x: Var, ty: Ty, body: Box<Tm> },
    App { t1: Box<Tm>, t2: Box<Tm> },
    Tru,
    Fls,
}

pub type Context = Map<Var, Ty>;


pub open spec fn type_check(ctx: Context, t: Tm) -> Option<Ty>
    decreases t
{
    match t {
        // TC_Var: lookup variable in context
        Tm::Var { x } => ctx_lookup(ctx, x),

        // TC_Abs: check body with extended context
        Tm::Abs { x, ty, body } => {
            match type_check(ctx_extend(ctx, x, ty), *body) {
                Option::None => Option::None,
                Option::Some(ty_body) => Option::Some(Ty::TArrow { t1: Box::new(ty), t2: Box::new(ty_body) }),
            }
        }

        // TC_App: check function and argument types match
        Tm::App { t1, t2 } => {
            match type_check(ctx, *t1) {
                Option::None => Option::None,
                Option::Some(ty1) => {
                    match ty1 {
                        Ty::TArrow { t1: ty_arg, t2: ty_ret } => {
                            match type_check(ctx, *t2) {
                                Option::None => Option::None,
                                Option::Some(ty2) => {
                                    if ty_eq(*ty_arg, ty2) {
                                        Option::Some(*ty_ret)
                                    } else {
                                        Option::None
                                    }
                                }
                            }
                        }
                        _ => Option::None,  // Not a function type
                    }
                }
            }
        }

        // TC_True, TC_False
        Tm::Tru => Option::Some(Ty::TBool),
        Tm::Fls => Option::Some(Ty::TBool),
    }
}

} // verus!