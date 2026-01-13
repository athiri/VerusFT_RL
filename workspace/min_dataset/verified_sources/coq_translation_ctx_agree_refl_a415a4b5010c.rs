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

pub open spec fn ctx_agree(ctx1: Context, ctx2: Context, vars: Set<Id>) -> bool {
    forall|x: Id| vars.contains(x) ==>
        (ctx_contains(ctx1, x) <==> ctx_contains(ctx2, x)) &&
        (ctx_contains(ctx1, x) ==> ctx1[x] == ctx2[x])
}


pub proof fn ctx_agree_refl(ctx: Context, vars: Set<Id>)
    ensures ctx_agree(ctx, ctx, vars)
{
}

} // verus!