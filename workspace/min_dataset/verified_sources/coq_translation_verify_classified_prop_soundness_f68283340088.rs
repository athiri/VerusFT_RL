use vstd::prelude::*;

verus! {

pub open spec fn classify_nat_size(n: nat) -> SizeCategory {
    if n <= 5 {
        SizeCategory::Tiny
    } else if n <= 20 {
        SizeCategory::Small
    } else if n <= 100 {
        SizeCategory::Medium
    } else if n <= 1000 {
        SizeCategory::Large
    } else {
        SizeCategory::Huge
    }
}


pub struct ClassifiedProp<C> {
    pub category: C,
    pub passed: bool,
    pub value: int,  // The test value (for statistics)
}

pub enum SizeCategory {
    Tiny,       // 0-5
    Small,      // 6-20
    Medium,     // 21-100
    Large,      // 101-1000
    Huge,       // > 1000
}

pub open spec fn classify_with_size(n: nat, prop: bool) -> ClassifiedProp<SizeCategory> {
    ClassifiedProp {
        category: classify_nat_size(n),
        passed: prop,
        value: n as int,
    }
}


pub proof fn verify_classified_prop_soundness(n: nat, prop: bool)
    ensures
        classify_with_size(n, prop).passed == prop &&
        classify_with_size(n, prop).value == n
{
}

} // verus!