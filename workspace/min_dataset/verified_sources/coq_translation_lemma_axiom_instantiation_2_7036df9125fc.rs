use vstd::prelude::*;

verus! {

pub open spec fn f(x: int) -> int { arbitrary() }
pub open spec fn g(x: int) -> int { arbitrary() }

pub broadcast axiom fn axiom_fg(x: int)
    ensures #[trigger] f(x) == g(x)
;

pub uninterp spec fn p(x: int) -> bool;

pub broadcast axiom fn axiom_p_implies_p(x: int)
    ensures #[trigger] p(x) ==> p(x)
;

pub proof fn lemma_forall_with_explicit_trigger()
    ensures forall|x: int| #[trigger] f(x) == f(x)
{
}

pub proof fn lemma_axiom_instantiation_2(t: int)
    ensures f(t) == g(t)
{
    axiom_fg(t);
    assert(f(t) == g(t));
}

} // verus!
