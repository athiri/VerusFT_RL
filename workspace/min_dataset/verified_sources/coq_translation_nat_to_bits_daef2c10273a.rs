use vstd::prelude::*;

verus! {

pub open spec fn nat_to_bits(n: nat, len: nat) -> Seq<bool>
    decreases len
{
    if len == 0 {
        Seq::empty()
    } else {
        nat_to_bits(n / 2, (len - 1) as nat).push(n % 2 == 1)
    }
}

} // verus!