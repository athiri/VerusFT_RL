use vstd::prelude::*;

verus! {

pub open spec fn ctx_contains(ctx: Context, x: Id) -> bool {
    ctx.dom().contains(x)
}


pub type Id = nat;

pub enum Ty {
    TBool,
    TNat,
    TArrow { t1: Box<Ty>, t2: Box<Ty> },
}

pub type Context = Map<Id, Ty>;

pub open spec fn ctx_included(ctx1: Context, ctx2: Context) -> bool {
    forall|x: Id| ctx_contains(ctx1, x) ==> ctx_contains(ctx2, x) && ctx1[x] == ctx2[x]
}


pub proof fn ctx_included_refl(ctx: Context)
    ensures ctx_included(ctx, ctx)
{
}

} // verus!