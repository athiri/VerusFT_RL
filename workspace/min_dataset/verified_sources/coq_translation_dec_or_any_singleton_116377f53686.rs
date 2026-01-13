use vstd::prelude::*;

verus! {

pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_or_any(ds: Seq<Dec>) -> Dec {
    dec_or_any_helper(ds, 0)
}

pub open spec fn dec_or_any_helper(ds: Seq<Dec>, i: int) -> Dec
    decreases ds.len() - i when i >= 0
{
    if i >= ds.len() {
        Dec::No
    } else {
        match ds[i] {
            Dec::Yes => Dec::Yes,
            Dec::No => dec_or_any_helper(ds, i + 1),
        }
    }
}


pub proof fn dec_or_any_singleton(d: Dec)
    ensures dec_or_any(seq![d]) == d
{
    reveal_with_fuel(dec_or_any_helper, 2);
}

} // verus!