use vstd::prelude::*;

verus! {

pub open spec fn f(x: nat) -> nat {
    x + 1
}

pub open spec fn p(x: int) -> bool { arbitrary() }

pub broadcast axiom fn axiom_p_implies_p(x: int)
    ensures #[trigger] p(x) ==> p(x)
;

pub proof fn lemma_forall_with_explicit_trigger()
    ensures forall|x: nat| #[trigger] f(x) == f(x)
{
}

pub proof fn lemma_forall_p_is_trivial()
    ensures forall|x: int| #[trigger] p(x) ==> p(x)
{
    assert(forall|x: int| #[trigger] p(x) ==> p(x));
}

} // verus!
