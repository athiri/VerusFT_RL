use vstd::prelude::*;

fn main() {}

verus! {

proof fn array_eq_reflexive<const N: usize>(a: [u8; N])
    ensures
        a@ =~= a@,
{
}

}
