// Variation: Range Operations
// From: source/verismo/src/mem/rawmem_p.rs (range checking)
// Demonstrates: Range containment and disjoint checking

use vstd::prelude::*;

verus! {

pub open spec fn within_range(addr: usize, range: (usize, usize)) -> bool {
    range.0 <= addr && addr < range.0 + range.1
}

pub open spec fn inside_range(inner: (usize, usize), outer: (usize, usize)) -> bool {
    outer.0 <= inner.0 && inner.0 + inner.1 <= outer.0 + outer.1
}

pub open spec fn ranges_disjoint(r1: (usize, usize), r2: (usize, usize)) -> bool {
    r1.0 + r1.1 <= r2.0 || r2.0 + r2.1 <= r1.0
}

pub fn check_within_range(addr: usize, range: (usize, usize)) -> (result: bool)
    requires
        range.0 <= usize::MAX - range.1,
    ensures
        result <==> within_range(addr, range),
{
    range.0 <= addr && addr < range.0 + range.1
}

pub fn check_inside_range(inner: (usize, usize), outer: (usize, usize)) -> (result: bool)
    requires
        inner.0 <= usize::MAX - inner.1,
        outer.0 <= usize::MAX - outer.1,
    ensures
        result <==> inside_range(inner, outer),
{
    outer.0 <= inner.0 && inner.0 + inner.1 <= outer.0 + outer.1
}

pub fn check_ranges_disjoint(r1: (usize, usize), r2: (usize, usize)) -> (result: bool)
    requires
        r1.0 <= usize::MAX - r1.1,
        r2.0 <= usize::MAX - r2.1,
    ensures
        result <==> ranges_disjoint(r1, r2),
{
    r1.0 + r1.1 <= r2.0 || r2.0 + r2.1 <= r1.0
}

pub fn ranges_overlap(r1: (usize, usize), r2: (usize, usize)) -> (result: bool)
    requires
        r1.0 <= usize::MAX - r1.1,
        r2.0 <= usize::MAX - r2.1,
    ensures
        result <==> !ranges_disjoint(r1, r2),
{
    !check_ranges_disjoint(r1, r2)
}

pub fn range_start_before(r1: (usize, usize), r2: (usize, usize)) -> (result: bool)
    ensures
        result <==> r1.0 < r2.0,
{
    r1.0 < r2.0
}

pub fn range_size_larger(r1: (usize, usize), r2: (usize, usize)) -> (result: bool)
    ensures
        result <==> r1.1 > r2.1,
{
    r1.1 > r2.1
}

pub fn get_range_gap(r1: (usize, usize), r2: (usize, usize)) -> (result: Option<usize>)
    requires
        r1.0 <= usize::MAX - r1.1,
        r2.0 <= usize::MAX - r2.1,
    ensures
        match result {
            Some(gap) => ranges_disjoint(r1, r2) && (r1.0 + r1.1 <= r2.0 ==> gap == r2.0 - (r1.0 + r1.1)),
            None => !ranges_disjoint(r1, r2),
        },
{
    if check_ranges_disjoint(r1, r2) {
        if r1.0 + r1.1 <= r2.0 {
            Some(r2.0 - (r1.0 + r1.1))
        } else {
            Some(r1.0 - (r2.0 + r2.1))
        }
    } else {
        None
    }
}

fn test_range_operations() {
    let range1 = (0x1000usize, 0x100usize);
    let range2 = (0x2000usize, 0x100usize);
    let range3 = (0x1050usize, 0x50usize);

    let within1 = check_within_range(0x1050, range1);
    let within2 = check_within_range(0x2000, range1);

    let inside1 = check_inside_range(range3, range1);
    let inside2 = check_inside_range(range1, range3);

    let disjoint12 = check_ranges_disjoint(range1, range2);
    let disjoint13 = check_ranges_disjoint(range1, range3);

    let overlap12 = ranges_overlap(range1, range2);
    let overlap13 = ranges_overlap(range1, range3);

    let before12 = range_start_before(range1, range2);
    let larger12 = range_size_larger(range1, range3);

    let gap12 = get_range_gap(range1, range2);
    let gap13 = get_range_gap(range1, range3);
}

} // verus!

fn main() {
    test_range_operations();
}
