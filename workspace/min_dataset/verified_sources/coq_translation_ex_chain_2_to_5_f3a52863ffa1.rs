use vstd::prelude::*;

verus! {

pub open spec fn step1(x: nat, y: nat) -> bool {
    y == x + 1
}


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


pub proof fn ex_chain_2_to_5() -> (e: RtStep1)
    ensures rt_inv(e),
        rt_lhs(e) == 2,
        rt_rhs(e) == 5
{
    let e2 = lemma_rt_refl(2);
    let e3 = lemma_rt_snoc(e2, 3);
    let e4 = lemma_rt_snoc(e3, 4);
    lemma_rt_snoc(e4, 5)
}

} // verus!