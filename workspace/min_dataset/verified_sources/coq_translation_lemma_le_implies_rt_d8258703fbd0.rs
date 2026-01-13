use vstd::prelude::*;

verus! {

pub enum RtStep1 {
    Refl { x: nat },
    Snoc { prev: Box<RtStep1>, y: nat },
}

pub proof fn lemma_rt_refl(x: nat) -> (e: RtStep1)
    ensures rt_inv(e),
        rt_lhs(e) == x,
        rt_rhs(e) == x
{
    RtStep1::Refl { x }
}

pub proof fn lemma_rt_snoc(prev: RtStep1, y: nat) -> (e: RtStep1)
    requires rt_inv(prev),
        step1(rt_rhs(prev), y)
    ensures rt_inv(e),
        rt_lhs(e) == rt_lhs(prev),
        rt_rhs(e) == y
{
    RtStep1::Snoc { prev: Box::new(prev), y }
}

pub open spec fn rt_rhs(e: RtStep1) -> nat
    decreases e
{
    match e {
        RtStep1::Refl { x } => x,
        RtStep1::Snoc { prev: _, y } => y,
    }
}

pub open spec fn step1(x: nat, y: nat) -> bool {
    y == x + 1
}

pub open spec fn rt_lhs(e: RtStep1) -> nat
    decreases e
{
    match e {
        RtStep1::Refl { x } => x,
        RtStep1::Snoc { prev, y: _ } => rt_lhs(*prev),
    }
}

pub open spec fn rt_inv(e: RtStep1) -> bool
    decreases e
{
    match e {
        RtStep1::Refl { .. } => true,
        RtStep1::Snoc { prev, y } => rt_inv(*prev) && step1(rt_rhs(*prev), y),
    }
}


pub proof fn lemma_le_implies_rt(x: nat, y: nat) -> (e: RtStep1)
    requires x <= y
    ensures rt_inv(e),
        rt_lhs(e) == x,
        rt_rhs(e) == y
    decreases y
{
    if x == y {
        lemma_rt_refl(x)
    } else {
        // From x <= y and x != y we get x < y, so y > 0 and x <= y-1.
        assert(x < y);
        assert(y > 0);
        let y1 = (y - 1) as nat;
        assert(x <= y1);

        let prev = lemma_le_implies_rt(x, y1);
        // Extend by a final successor step from y-1 to y.
        assert(step1(y1, y));
        lemma_rt_snoc(prev, y)
    }
}

} // verus!