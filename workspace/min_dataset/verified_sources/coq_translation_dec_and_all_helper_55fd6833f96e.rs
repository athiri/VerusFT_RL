use vstd::prelude::*;

verus! {

pub enum Dec {
    Yes,
    No,
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

} // verus!