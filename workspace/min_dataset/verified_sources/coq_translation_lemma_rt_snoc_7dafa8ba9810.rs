use vstd::prelude::*;

verus! {

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

pub open spec fn step1(x: nat, y: nat) -> bool {
    y == x + 1
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

} // verus!