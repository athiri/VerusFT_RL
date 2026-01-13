use vstd::prelude::*;

verus! {

pub type Str = Seq<nat>;

pub open spec fn show_nat_helper(n: nat, acc: Str) -> Str
    decreases n
{
    if n == 0 && acc.len() > 0 {
        acc
    } else if n == 0 {
        seq![48nat]  // "0"
    } else {
        let digit = show_nat_digit(n % 10);
        show_nat_helper(n / 10, seq![digit] + acc)
    }
}

pub open spec fn show_nat_digit(n: nat) -> nat
    recommends n < 10
{
    48 + n  // ASCII '0' = 48
}


pub proof fn show_nat_helper_nonempty(n: nat, acc: Str)
    ensures show_nat_helper(n, acc).len() > 0
    decreases n
{
    reveal_with_fuel(show_nat_helper, 2);
    if n == 0 && acc.len() > 0 {
        // Returns acc, which has len > 0
    } else if n == 0 {
        // Returns seq![48nat] which has len 1
    } else {
        // Recursive case
        let digit = show_nat_digit(n % 10);
        show_nat_helper_nonempty(n / 10, seq![digit] + acc);
    }
}

} // verus!