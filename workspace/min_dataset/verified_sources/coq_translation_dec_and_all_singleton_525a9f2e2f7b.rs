use vstd::prelude::*;

verus! {

pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_and_all(ds: Seq<Dec>) -> Dec {
    dec_and_all_helper(ds, 0)
}

pub open spec fn dec_and_all_helper(ds: Seq<Dec>, i: int) -> Dec
    decreases ds.len() - i when i >= 0
{
    if i >= ds.len() {
        Dec::Yes
    } else {
        match ds[i] {
            Dec::No => Dec::No,
            Dec::Yes => dec_and_all_helper(ds, i + 1),
        }
    }
}


pub proof fn dec_and_all_singleton(d: Dec)
    ensures dec_and_all(seq![d]) == d
{
    reveal_with_fuel(dec_and_all_helper, 2);
}

} // verus!