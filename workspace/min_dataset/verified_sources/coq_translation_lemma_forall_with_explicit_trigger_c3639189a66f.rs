use vstd::prelude::*;

verus! {

pub open spec fn f(x: int) -> int { arbitrary() }
pub uninterp spec fn g(x: int) -> int;

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

} // verus!
