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

pub open spec fn size_cat_eq(c1: SizeCategory, c2: SizeCategory) -> bool {
    match (c1, c2) {
        (SizeCategory::Tiny, SizeCategory::Tiny) => true,
        (SizeCategory::Small, SizeCategory::Small) => true,
        (SizeCategory::Medium, SizeCategory::Medium) => true,
        (SizeCategory::Large, SizeCategory::Large) => true,
        (SizeCategory::Huge, SizeCategory::Huge) => true,
        _ => false,
    }
}


pub enum SizeCategory {
    Tiny,       // 0-5
    Small,      // 6-20
    Medium,     // 21-100
    Large,      // 101-1000
    Huge,       // > 1000
}


pub proof fn verify_classify_nat_size_coverage()
    ensures
        size_cat_eq(classify_nat_size(0), SizeCategory::Tiny) &&
        size_cat_eq(classify_nat_size(10), SizeCategory::Small) &&
        size_cat_eq(classify_nat_size(50), SizeCategory::Medium) &&
        size_cat_eq(classify_nat_size(500), SizeCategory::Large) &&
        size_cat_eq(classify_nat_size(5000), SizeCategory::Huge)
{
}

} // verus!