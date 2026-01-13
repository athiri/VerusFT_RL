use vstd::prelude::*;
use vstd::calc_macro::*;

verus! {

pub proof fn ex4_add_assoc(a: nat, b: nat, c: nat)
    ensures (a + b) + c == a + (b + c)
{}

pub proof fn ex_calc_nat_chain(a: nat, b: nat, c: nat)
    ensures (a + b) + c == c + (b + a)
{
    calc! {
        (==)
        (a + b) + c;
        { ex4_add_assoc(a, b, c); }
        a + (b + c);
        {}
        c + (b + a);
    }
}

} // verus!
