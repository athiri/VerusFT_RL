use vstd::prelude::*;

verus! {

pub open spec fn bool_to_dec(b: bool) -> Dec {
    if b { Dec::Yes } else { Dec::No }
}

pub open spec fn forall_nat_lt_helper(n: nat, p: spec_fn(nat) -> bool, i: nat) -> bool
    decreases n - i when i <= n
{
    if i >= n {
        true
    } else if !p(i) {
        false
    } else {
        forall_nat_lt_helper(n, p, i + 1)
    }
}


pub open spec fn forall_nat_lt(n: nat, p: spec_fn(nat) -> bool) -> bool {
    forall_nat_lt_helper(n, p, 0)
}

pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_to_bool(d: Dec) -> bool {
    match d {
        Dec::Yes => true,
        Dec::No => false,
    }
}

pub open spec fn dec_forall_nat_lt(n: nat, p: spec_fn(nat) -> bool) -> Dec {
    bool_to_dec(forall_nat_lt(n, p))
}


pub proof fn forall_false_reflect(n: nat, p: spec_fn(nat) -> bool)
    requires !dec_to_bool(dec_forall_nat_lt(n, p))
    ensures !forall_nat_lt(n, p)
{
}

} // verus!