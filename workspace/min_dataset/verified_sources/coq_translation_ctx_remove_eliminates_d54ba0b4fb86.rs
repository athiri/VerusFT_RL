use vstd::prelude::*;

verus! {

pub type Id = nat;

pub enum Ty {
    TBool,
    TNat,
    TArrow { t1: Box<Ty>, t2: Box<Ty> },
}

pub type Context = Map<Id, Ty>;

pub open spec fn ctx_remove(ctx: Context, x: Id) -> Context {
    ctx.remove(x)
}

pub open spec fn ctx_contains(ctx: Context, x: Id) -> bool {
    ctx.dom().contains(x)
}


pub proof fn ctx_remove_eliminates(ctx: Context, x: Id)
    ensures !ctx_contains(ctx_remove(ctx, x), x)
{
    assert(!ctx.remove(x).dom().contains(x));
}

} // verus!