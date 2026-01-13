use vstd::prelude::*;

verus! {

pub type Id = nat;

pub enum Ty {
    TBool,
    TNat,
    TArrow { t1: Box<Ty>, t2: Box<Ty> },
}

pub type Context = Map<Id, Ty>;

pub open spec fn ctx_extend(ctx: Context, x: Id, ty: Ty) -> Context {
    ctx.insert(x, ty)
}

pub open spec fn ctx_lookup(ctx: Context, x: Id) -> Option<Ty> {
    if ctx.dom().contains(x) {
        Option::Some(ctx[x])
    } else {
        Option::None
    }
}


pub proof fn ctx_extend_lookup(ctx: Context, x: Id, ty: Ty)
    ensures ctx_lookup(ctx_extend(ctx, x, ty), x) == Option::Some(ty)
{
    assert(ctx.insert(x, ty).dom().contains(x));
    assert(ctx.insert(x, ty)[x] == ty);
}

} // verus!