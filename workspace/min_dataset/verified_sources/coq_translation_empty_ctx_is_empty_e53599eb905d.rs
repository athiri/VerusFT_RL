use vstd::prelude::*;

verus! {

pub open spec fn empty_ctx() -> Context {
    Map::<Id, Ty>::empty()
}


pub type Id = nat;

pub enum Ty {
    TBool,
    TNat,
    TArrow { t1: Box<Ty>, t2: Box<Ty> },
}

pub type Context = Map<Id, Ty>;

pub open spec fn ctx_contains(ctx: Context, x: Id) -> bool {
    ctx.dom().contains(x)
}


pub proof fn empty_ctx_is_empty()
    ensures forall|x: Id| !ctx_contains(empty_ctx(), x)
{
    assert forall|x: Id| !ctx_contains(empty_ctx(), x) by {
        assert(!Map::<Id, Ty>::empty().dom().contains(x));
    }
}

} // verus!