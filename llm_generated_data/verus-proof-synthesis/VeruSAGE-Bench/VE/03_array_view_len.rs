use vstd::prelude::*;

fn main() {}

verus! {

proof fn array_view_len<const N: usize>(a: [u8; N])
    ensures
        a@.len() == N,
{
}

}
