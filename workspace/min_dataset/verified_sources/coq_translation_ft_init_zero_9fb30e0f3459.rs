use vstd::prelude::*;

verus! {

pub struct FenwickTree { pub tree: Seq<nat>, pub n: nat }

pub open spec fn ft_init(n: nat) -> FenwickTree {
    FenwickTree { tree: Seq::new(n, |_i: int| 0nat), n }
}

pub open spec fn prefix_sum(ft: FenwickTree, i: nat) -> nat
    decreases i
{
    if i == 0 { 0 }
    else if i > ft.n { 0 }
    else { ft.tree[(i - 1) as int] + prefix_sum(ft, (i - 1) as nat) }
}


pub proof fn ft_init_zero(n: nat)
    ensures prefix_sum(ft_init(n), 0) == 0
{
    reveal_with_fuel(prefix_sum, 2);
}

} // verus!