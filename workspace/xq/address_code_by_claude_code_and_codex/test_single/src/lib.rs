// Simple test to verify the syntax fixes work
use vstd::prelude::*;

verus! {

// Test 1: matches syntax works
pub enum Popped {
    No,
    Ready(int),
    VeryUnready(int, int),
}

pub proof fn test_matches(p: Popped) {
    // Old syntax: p.is_VeryUnready()
    // New syntax: p matches Popped::VeryUnready(..)
    if p matches Popped::VeryUnready(..) {
        assert(!(p matches Popped::No));
    }
}

// Test 2: matches with ==> works (needs parentheses)
pub proof fn test_matches_with_implication(p: Popped, condition: bool) {
    // This should compile without syntax errors
    assert(condition ==> (p matches Popped::VeryUnready(..)) ==> (p matches Popped::VeryUnready(..)));
}

// Test 3: Simple enum without #[is_variant]
pub enum TestEnum {
    Variant1,
    Variant2(int),
}

pub fn test_enum_works(e: TestEnum) -> bool {
    e matches TestEnum::Variant2(..)
}

} // verus!

fn main() {}
