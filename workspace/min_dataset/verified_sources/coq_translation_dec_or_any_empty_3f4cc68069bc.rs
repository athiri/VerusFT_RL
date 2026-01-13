use vstd::prelude::*;

verus! {

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


pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_or_any(ds: Seq<Dec>) -> Dec {
    dec_or_any_helper(ds, 0)
}


pub proof fn dec_or_any_empty()
    ensures dec_or_any(Seq::<Dec>::empty()) == Dec::No
{
}

} // verus!