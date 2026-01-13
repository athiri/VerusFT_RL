use vstd::prelude::*;

verus! {

pub open spec fn exists_range_helper(lo: nat, hi: nat, p: spec_fn(nat) -> bool, i: nat) -> bool
    decreases hi - i when i <= hi
{
    if i >= hi {
        false
    } else if p(i) {
        true
    } else {
        exists_range_helper(lo, hi, p, i + 1)
    }
}

pub open spec fn bool_to_dec(b: bool) -> Dec {
    if b { Dec::Yes } else { Dec::No }
}


pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_exists_range(lo: nat, hi: nat, p: spec_fn(nat) -> bool) -> Dec {
    bool_to_dec(exists_range(lo, hi, p))
}

pub open spec fn dec_to_bool(d: Dec) -> bool {
    match d {
        Dec::Yes => true,
        Dec::No => false,
    }
}

pub open spec fn exists_range(lo: nat, hi: nat, p: spec_fn(nat) -> bool) -> bool {
    exists_range_helper(lo, hi, p, lo)
}


pub proof fn dec_exists_range_sound(lo: nat, hi: nat, p: spec_fn(nat) -> bool)
    ensures dec_to_bool(dec_exists_range(lo, hi, p)) == exists_range(lo, hi, p)
{
}

} // verus!