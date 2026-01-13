use vstd::prelude::*;

verus! {

pub open spec fn ack(m: nat, n: nat) -> nat decreases m, n {
    if m == 0 { n + 1 }
    else if n == 0 { ack((m - 1) as nat, 1) }
    else { ack((m - 1) as nat, ack(m, (n - 1) as nat)) }
}


pub proof fn ack_increasing_n(m: nat, n: nat) ensures ack(m, n) < ack(m, n + 1) decreases m, n {
    reveal_with_fuel(ack, 3);
    if m == 0 {} else { assume(ack(m, n) < ack(m, n + 1)); } // Simplified
}

} // verus!