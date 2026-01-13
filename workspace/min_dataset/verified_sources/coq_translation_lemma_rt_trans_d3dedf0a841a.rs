use vstd::prelude::*;

verus! {

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


pub enum RtStep1 {
    Refl { x: nat },
    Snoc { prev: Box<RtStep1>, y: nat },
}

pub open spec fn rt_inv(e: RtStep1) -> bool
    decreases e
{
    match e {
        RtStep1::Refl { .. } => true,
        RtStep1::Snoc { prev, y } => rt_inv(*prev) && step1(rt_rhs(*prev), y),
    }
}


pub proof fn lemma_rt_trans(e1: RtStep1, e2: RtStep1) -> (e: RtStep1)
    requires rt_inv(e1),
        rt_inv(e2),
        rt_rhs(e1) == rt_lhs(e2)
    ensures rt_inv(e),
        rt_lhs(e) == rt_lhs(e1),
        rt_rhs(e) == rt_rhs(e2)
    decreases e2
{
    match e2 {
        RtStep1::Refl { x: _ } => {
            // e2 is reflexive at rt_lhs(e2)=rt_rhs(e2), so result is e1.
            e1
        }
        RtStep1::Snoc { prev, y } => {
            // First, concatenate e1 with the prefix.
            let prev_e2 = *prev;
            let mid = lemma_rt_trans(e1, prev_e2);

            // Now extend with the last step.
            // From rt_inv(e2): step1(rt_rhs(prev), y)
            // and from the recursion: rt_rhs(mid) == rt_rhs(prev).
            assert(step1(rt_rhs(mid), y));
            lemma_rt_snoc(mid, y)
        }
    }
}

} // verus!