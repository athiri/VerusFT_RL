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


pub open spec fn ctx_extend_many(ctx: Context, bindings: Seq<(Id, Ty)>) -> Context
    decreases bindings.len()
{
    if bindings.len() == 0 {
        ctx
    } else {
        let (x, ty) = bindings[0];
        ctx_extend_many(ctx_extend(ctx, x, ty), bindings.skip(1))
    }
}

} // verus!