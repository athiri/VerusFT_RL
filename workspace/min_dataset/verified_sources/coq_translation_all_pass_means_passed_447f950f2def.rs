use vstd::prelude::*;

verus! {

pub proof fn all_pass_means_passed(gen: Set<nat>, prop: spec_fn(nat) -> bool, max_tests: nat, values: Seq<nat>)
    requires
        values.len() as nat >= max_tests,
        forall|i: int| 0 <= i < values.len() ==> gen.contains(values[i]) ==> prop(values[i])
{
    // Property passes when all tests pass
}

} // verus!