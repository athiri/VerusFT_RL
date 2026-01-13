use vstd::prelude::*;

verus! {

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

pub open spec fn empty_ctx() -> Context {
    Map::<Id, Ty>::empty()
}

pub open spec fn ctx_contains(ctx: Context, x: Id) -> bool {
    ctx.dom().contains(x)
}


pub proof fn empty_ctx_included(ctx: Context)
    ensures ctx_included(empty_ctx(), ctx)
{
    assert forall|x: Id| ctx_contains(empty_ctx(), x) implies ctx_contains(ctx, x) && empty_ctx()[x] == ctx[x] by {
        assert(!ctx_contains(empty_ctx(), x));
    }
}

} // verus!