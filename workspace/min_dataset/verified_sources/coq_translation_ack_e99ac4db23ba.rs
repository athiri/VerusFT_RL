use vstd::prelude::*;

verus! {

pub open spec fn ack(m: nat, n: nat) -> nat decreases m, n {
    if m == 0 { n + 1 }
    else if n == 0 { ack((m - 1) as nat, 1) }
    else { ack((m - 1) as nat, ack(m, (n - 1) as nat)) }
}

} // verus!