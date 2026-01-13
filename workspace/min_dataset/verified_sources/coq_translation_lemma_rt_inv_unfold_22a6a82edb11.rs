use vstd::prelude::*;

verus! {

pub enum RtStep1 {
    Refl { x: nat },
    Snoc { prev: Box<RtStep1>, y: nat },
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

pub open spec fn rt_rhs(e: RtStep1) -> nat
    decreases e
{
    match e {
        RtStep1::Refl { x } => x,
        RtStep1::Snoc { prev: _, y } => y,
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



pub proof fn lemma_rt_inv_unfold(e: RtStep1)
    ensures rt_inv(e) == match e {
        RtStep1::Refl { .. } => true,
        RtStep1::Snoc { prev, y } => rt_inv(*prev) && step1(rt_rhs(*prev), y),
    }
{}

} // verus!