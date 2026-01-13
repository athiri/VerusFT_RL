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

pub open spec fn ctx_contains(ctx: Context, x: Id) -> bool {
    ctx.dom().contains(x)
}


pub proof fn ctx_extend_contains(ctx: Context, x: Id, ty: Ty)
    ensures ctx_contains(ctx_extend(ctx, x, ty), x)
{
    assert(ctx.insert(x, ty).dom().contains(x));
}

} // verus!