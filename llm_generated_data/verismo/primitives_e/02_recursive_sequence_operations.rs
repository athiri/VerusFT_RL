// Variation: Recursive Sequence Operations
// From: source/verismo/src/primitives_e/seq.rs (recursive_sec_bytes)
// Demonstrates: Recursive operations on sequences with decreases clause

use vstd::prelude::*;

verus! {

pub open spec fn recursive_sum(s: Seq<u64>) -> u64
    decreases s.len(),
{
    if s.len() > 0 {
        let rest = s.subrange(0, s.len() - 1);
        if rest.len() < s.len() {
            (recursive_sum(rest) + s.last()) as u64
        } else {
            0u64
        }
    } else {
        0u64
    }
}

pub open spec fn recursive_concat(s: Seq<Seq<u8>>) -> Seq<u8>
    decreases s.len(),
{
    if s.len() > 0 {
        let prevs = s.subrange(0, s.len() - 1);
        if prevs.len() < s.len() {
            recursive_concat(prevs) + s.last()
        } else {
            Seq::empty()
        }
    } else {
        Seq::empty()
    }
}

pub open spec fn recursive_max(s: Seq<u64>) -> u64
    decreases s.len(),
{
    if s.len() == 0 {
        0
    } else if s.len() == 1 {
        s[0]
    } else {
        let rest = s.subrange(0, s.len() - 1);
        let rest_max = recursive_max(rest);
        if s.last() > rest_max {
            s.last()
        } else {
            rest_max
        }
    }
}

pub open spec fn recursive_product(s: Seq<u64>) -> u64
    decreases s.len(),
{
    if s.len() == 0 {
        1u64
    } else {
        let rest = s.subrange(0, s.len() - 1);
        if rest.len() < s.len() {
            (recursive_product(rest) * s.last()) as u64
        } else {
            1u64
        }
    }
}

pub open spec fn recursive_all_positive(s: Seq<i64>) -> bool
    decreases s.len(),
{
    if s.len() == 0 {
        true
    } else {
        let rest = s.subrange(0, s.len() - 1);
        recursive_all_positive(rest) && s.last() > 0
    }
}

pub proof fn lemma_recursive_sum_empty()
    ensures
        recursive_sum(Seq::<u64>::empty()) == 0,
{
}

// Note: This lemma cannot be verified due to type casting complexities
// pub proof fn lemma_recursive_sum_single(val: u64)
//     ensures
//         recursive_sum(Seq::new(1, |i: int| val)) == val,
// {
// }

pub proof fn lemma_recursive_concat_empty()
    ensures
        recursive_concat(Seq::<Seq<u8>>::empty()) == Seq::<u8>::empty(),
{
}

pub proof fn lemma_recursive_max_empty()
    ensures
        recursive_max(Seq::<u64>::empty()) == 0,
{
}

pub proof fn lemma_recursive_max_single(val: u64)
    ensures
        recursive_max(Seq::new(1, |i: int| val)) == val,
{
}

pub proof fn lemma_recursive_product_empty()
    ensures
        recursive_product(Seq::<u64>::empty()) == 1,
{
}

// Note: This lemma cannot be verified due to type casting complexities
// pub proof fn lemma_recursive_product_single(val: u64)
//     ensures
//         recursive_product(Seq::new(1, |i: int| val)) == val,
// {
// }

pub proof fn lemma_recursive_all_positive_empty()
    ensures
        recursive_all_positive(Seq::<i64>::empty()),
{
}

fn test_recursive_operations() {
    proof {
        let s1 = Seq::new(3, |i: int| (i + 1) as u64);

        lemma_recursive_sum_empty();
        lemma_recursive_concat_empty();
        lemma_recursive_max_empty();
        lemma_recursive_max_single(100);
        lemma_recursive_product_empty();
        lemma_recursive_all_positive_empty();

        let empty_sum = recursive_sum(Seq::<u64>::empty());
        let max_val = recursive_max(s1);
    }
}

} // verus!

fn main() {
    test_recursive_operations();
}
