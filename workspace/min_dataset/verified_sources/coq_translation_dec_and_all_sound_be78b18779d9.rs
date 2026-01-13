use vstd::prelude::*;

verus! {

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

pub open spec fn dec_and_all(ds: Seq<Dec>) -> Dec {
    dec_and_all_helper(ds, 0)
}


pub proof fn dec_and_all_sound(ds: Seq<Dec>)
    ensures dec_to_bool(dec_and_all(ds)) == dec_to_bool(dec_and_all_helper(ds, 0))
{
}

} // verus!