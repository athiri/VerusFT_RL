use vstd::prelude::*;

verus! {

pub open spec fn shrink_nat(n: nat) -> Seq<nat>
    decreases n
{
    if n == 0 {
        Seq::empty()
    } else {
        let half = (n / 2) as nat;
        if half == 0 {
            seq![0nat]
        } else {
            seq![0nat, half]
        }
    }
}


pub open spec fn shrink_option_nat(o: Option<nat>) -> Seq<Option<nat>> {
    match o {
        Option::None => Seq::empty(),
        Option::Some(n) => {
            // Shrink to None first, then shrink the inner value
            let shrunk_inner = shrink_nat(n);
            // Add Some(x) for each shrunk inner value
            Seq::new(shrunk_inner.len() + 1, |i: int|
                if i == 0 {
                    Option::<nat>::None
                } else {
                    Option::Some(shrunk_inner[i - 1])
                }
            )
        }
    }
}


pub proof fn shrink_option_none()
    ensures shrink_option_nat(Option::None).len() == 0
{
    // Option::None doesn't shrink
}

} // verus!