use vstd::prelude::*;

verus! {

pub enum SizeCategory {
    Tiny,       // 0-5
    Small,      // 6-20
    Medium,     // 21-100
    Large,      // 101-1000
    Huge,       // > 1000
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

pub struct ClassifiedProp<C> {
    pub category: C,
    pub passed: bool,
    pub value: int,  // The test value (for statistics)
}


pub open spec fn count_by_size_helper(props: Seq<ClassifiedProp<SizeCategory>>, cat: SizeCategory, idx: int) -> nat
    decreases props.len() - idx
{
    if idx >= props.len() {
        0
    } else {
        let count = if size_cat_eq(props[idx].category, cat) { 1 as nat } else { 0 as nat };
        count + count_by_size_helper(props, cat, idx + 1)
    }
}

} // verus!