#[allow(unused_imports)]
#[allow(unused_imports)]

use vstd::prelude::*;

verus! {

proof fn lemma() {
    let a: Seq<nat> = seq![1, 2, 3];
    assert(a[1] == 2);
}

fn main() {
}

} // verus!
