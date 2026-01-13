use vstd::prelude::*;

verus! {

pub open spec fn ack(m: nat, n: nat) -> nat decreases m, n {
    if m == 0 { n + 1 }
    else if n == 0 { ack((m - 1) as nat, 1) }
    else { ack((m - 1) as nat, ack(m, (n - 1) as nat)) }
}


pub proof fn ack_positive(m: nat, n: nat) ensures ack(m, n) > 0 decreases m, n {
    reveal_with_fuel(ack, 2);
    if m == 0 {} else if n == 0 { ack_positive((m - 1) as nat, 1); } else { ack_positive(m, (n - 1) as nat); ack_positive((m - 1) as nat, ack(m, (n - 1) as nat)); }
}

} // verus!