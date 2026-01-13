use vstd::prelude::*;

verus! {

pub open spec fn exists_nat_lt_helper(n: nat, p: spec_fn(nat) -> bool, i: nat) -> bool
    decreases n - i when i <= n
{
    if i >= n {
        false
    } else if p(i) {
        true
    } else {
        exists_nat_lt_helper(n, p, i + 1)
    }
}


pub open spec fn bool_to_dec(b: bool) -> Dec {
    if b { Dec::Yes } else { Dec::No }
}

pub open spec fn exists_nat_lt(n: nat, p: spec_fn(nat) -> bool) -> bool {
    exists_nat_lt_helper(n, p, 0)
}


pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_exists_nat_lt(n: nat, p: spec_fn(nat) -> bool) -> Dec {
    bool_to_dec(exists_nat_lt(n, p))
}

pub open spec fn dec_to_bool(d: Dec) -> bool {
    match d {
        Dec::Yes => true,
        Dec::No => false,
    }
}


pub proof fn exists_empty_range(p: spec_fn(nat) -> bool)
    ensures !dec_to_bool(dec_exists_nat_lt(0, p))
{
}

} // verus!