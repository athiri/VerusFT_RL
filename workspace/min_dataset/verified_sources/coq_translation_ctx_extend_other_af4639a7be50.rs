use vstd::prelude::*;

verus! {

pub open spec fn ctx_extend(ctx: Context, x: Id, ty: Ty) -> Context {
    ctx.insert(x, ty)
}


pub type Id = nat;

pub enum Ty {
    TBool,
    TNat,
    TArrow { t1: Box<Ty>, t2: Box<Ty> },
}

pub type Context = Map<Id, Ty>;

pub open spec fn ctx_lookup(ctx: Context, x: Id) -> Option<Ty> {
    if ctx.dom().contains(x) {
        Option::Some(ctx[x])
    } else {
        Option::None
    }
}


pub proof fn ctx_extend_other(ctx: Context, x: Id, y: Id, ty: Ty)
    requires x != y
    ensures ctx_lookup(ctx_extend(ctx, x, ty), y) == ctx_lookup(ctx, y)
{
    if ctx.dom().contains(y) {
        assert(ctx.insert(x, ty).dom().contains(y));
        assert(ctx.insert(x, ty)[y] == ctx[y]);
    } else {
        assert(!ctx.insert(x, ty).dom().contains(y));
    }
}

} // verus!