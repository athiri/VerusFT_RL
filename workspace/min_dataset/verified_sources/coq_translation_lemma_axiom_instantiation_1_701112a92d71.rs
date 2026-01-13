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

pub proof fn lemma_axiom_instantiation_1()
    ensures f(7) == g(7)
{
    axiom_fg(7);
    assert(f(7) == g(7));
}

} // verus!
