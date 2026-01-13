use vstd::prelude::*;

verus! {

pub enum RtStep1 {
    Refl { x: nat },
    Snoc { prev: Box<RtStep1>, y: nat },
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


pub proof fn lemma_rt_implies_le(e: RtStep1)
    requires rt_inv(e)
    ensures rt_lhs(e) <= rt_rhs(e)
    decreases e
{
    match e {
        RtStep1::Refl { x: _ } => {
        }
        RtStep1::Snoc { prev, y } => {
            let p = *prev;
            lemma_rt_implies_le(p);
            // rt_inv(e) implies step1(rt_rhs(p), y), hence y = rt_rhs(p) + 1.
            assert(step1(rt_rhs(p), y));
            assert(rt_rhs(p) < y);
            // Combine: rt_lhs(p) <= rt_rhs(p) < y, and rt_lhs(e)=rt_lhs(p), rt_rhs(e)=y.
            assert(rt_lhs(p) <= y);
        }
    }
}

} // verus!