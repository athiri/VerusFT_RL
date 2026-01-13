use vstd::prelude::*;

verus! {

pub enum Dec {
    Yes,
    No,
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

} // verus!