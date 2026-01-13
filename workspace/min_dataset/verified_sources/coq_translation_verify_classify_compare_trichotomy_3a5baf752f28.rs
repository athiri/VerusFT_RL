use vstd::prelude::*;

verus! {

pub enum CompareCategory {
    Less,
    Equal,
    Greater,
}

pub open spec fn classify_compare(x: nat, y: nat) -> CompareCategory {
    if x < y {
        CompareCategory::Less
    } else if x == y {
        CompareCategory::Equal
    } else {
        CompareCategory::Greater
    }
}

pub open spec fn compare_cat_eq(c1: CompareCategory, c2: CompareCategory) -> bool {
    match (c1, c2) {
        (CompareCategory::Less, CompareCategory::Less) => true,
        (CompareCategory::Equal, CompareCategory::Equal) => true,
        (CompareCategory::Greater, CompareCategory::Greater) => true,
        _ => false,
    }
}


pub proof fn verify_classify_compare_trichotomy(x: nat, y: nat)
    ensures
        (x < y ==> compare_cat_eq(classify_compare(x, y), CompareCategory::Less)) &&
        (x == y ==> compare_cat_eq(classify_compare(x, y), CompareCategory::Equal)) &&
        (x > y ==> compare_cat_eq(classify_compare(x, y), CompareCategory::Greater))
{
}

} // verus!