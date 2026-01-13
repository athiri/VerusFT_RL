use vstd::prelude::*;
use vstd::calc_macro::*;

verus! {

pub proof fn lemma_nat_add_assoc(a: nat, b: nat, c: nat)
    ensures a + (b + c) == (a + b) + c
{}

pub proof fn lemma_nat_add_comm(a: nat, b: nat)
    ensures a + b == b + a
{}

pub proof fn ex_calc_chain(a: nat, b: nat, c: nat)
    ensures a + (b + c) == c + (b + a)
{
    calc! {
        (==)
        a + (b + c);
        { lemma_nat_add_assoc(a, b, c); }
        (a + b) + c;
        { lemma_nat_add_comm(a + b, c); }
        c + (a + b);
        {
            lemma_nat_add_comm(a, b);
            assert(c + (a + b) == c + (b + a));
        }
        c + (b + a);
    }
}

} // verus!
