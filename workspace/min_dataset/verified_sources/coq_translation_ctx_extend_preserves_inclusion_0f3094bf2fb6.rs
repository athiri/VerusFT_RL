use vstd::prelude::*;

verus! {

pub open spec fn ctx_extend(ctx: Context, x: Id, ty: Ty) -> Context {
    ctx.insert(x, ty)
}

pub open spec fn ctx_included(ctx1: Context, ctx2: Context) -> bool {
    forall|x: Id| ctx_contains(ctx1, x) ==> ctx_contains(ctx2, x) && ctx1[x] == ctx2[x]
}

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


pub proof fn ctx_extend_preserves_inclusion(ctx1: Context, ctx2: Context, x: Id, ty: Ty)
    requires
        ctx_included(ctx1, ctx2),
        !ctx_contains(ctx1, x),
    ensures ctx_included(ctx1, ctx_extend(ctx2, x, ty))
{
    assert forall|y: Id| ctx_contains(ctx1, y) implies ctx_contains(ctx_extend(ctx2, x, ty), y) && ctx1[y] == ctx_extend(ctx2, x, ty)[y] by {
        if ctx_contains(ctx1, y) {
            assert(y != x);  // Because !ctx_contains(ctx1, x)
            assert(ctx_contains(ctx2, y));
            assert(ctx1[y] == ctx2[y]);
            assert(ctx_extend(ctx2, x, ty)[y] == ctx2[y]);
        }
    }
}

} // verus!