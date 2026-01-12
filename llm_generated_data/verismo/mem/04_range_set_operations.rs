// Variation: Range Set Operations
// From: source/verismo/src/mem/rawmem_p.rs (range set concepts)
// Demonstrates: Operations on sets of memory ranges

use vstd::prelude::*;

verus! {

pub open spec fn range_to_set(range: (usize, usize)) -> Set<usize> {
    Set::new(|addr: usize| range.0 <= addr && addr < range.0 + range.1)
}

pub struct RangeSet {
    pub ranges: Vec<(usize, usize)>,
}

impl RangeSet {
    pub fn new() -> (result: Self)
        ensures
            result.ranges@.len() == 0,
    {
        RangeSet { ranges: Vec::new() }
    }

    pub fn add_range(&mut self, range: (usize, usize))
        requires
            range.1 > 0,
        ensures
            self.ranges@.len() == old(self).ranges@.len() + 1,
    {
        self.ranges.push(range);
    }

    pub fn count(&self) -> (result: usize)
        ensures
            result == self.ranges@.len(),
    {
        self.ranges.len()
    }

    pub fn is_empty(&self) -> (result: bool)
        ensures
            result <==> self.ranges@.len() == 0,
    {
        self.ranges.len() == 0
    }

    pub fn contains_range(&self, range: (usize, usize)) -> (result: bool)
    {
        let mut i = 0;
        while i < self.ranges.len()
            invariant
                0 <= i <= self.ranges@.len(),
            decreases self.ranges@.len() - i
        {
            if self.ranges[i].0 == range.0 && self.ranges[i].1 == range.1 {
                return true;
            }
            i = i + 1;
        }
        false
    }

    pub fn get_total_size(&self) -> (result: usize)
    {
        let mut total = 0;
        let mut i = 0;
        while i < self.ranges.len()
            invariant
                0 <= i <= self.ranges@.len(),
            decreases self.ranges@.len() - i
        {
            if total <= usize::MAX - self.ranges[i].1 {
                total = total + self.ranges[i].1;
            }
            i = i + 1;
        }
        total
    }

    pub fn get_min_start(&self) -> (result: Option<usize>)
        requires
            self.ranges@.len() > 0,
    {
        let mut min_start = self.ranges[0].0;
        let mut i = 1;
        while i < self.ranges.len()
            invariant
                1 <= i <= self.ranges@.len(),
            decreases self.ranges@.len() - i
        {
            if self.ranges[i].0 < min_start {
                min_start = self.ranges[i].0;
            }
            i = i + 1;
        }
        Some(min_start)
    }

    pub fn get_max_end(&self) -> (result: Option<usize>)
        requires
            self.ranges@.len() > 0,
    {
        let mut max_end = 0;
        let mut i = 0;
        while i < self.ranges.len()
            invariant
                0 <= i <= self.ranges@.len(),
            decreases self.ranges@.len() - i
        {
            let r = self.ranges[i];
            if r.0 <= usize::MAX - r.1 {
                let end = r.0 + r.1;
                if end > max_end {
                    max_end = end;
                }
            }
            i = i + 1;
        }
        Some(max_end)
    }
}

fn test_range_set_operations() {
    let mut rset = RangeSet::new();

    let empty = rset.is_empty();
    let count0 = rset.count();

    rset.add_range((0x1000, 0x100));
    rset.add_range((0x2000, 0x200));
    rset.add_range((0x3000, 0x50));

    let count3 = rset.count();

    let contains1 = rset.contains_range((0x1000, 0x100));
    let contains2 = rset.contains_range((0x5000, 0x100));

    let total_size = rset.get_total_size();

    let min_start = rset.get_min_start();
    let max_end = rset.get_max_end();
}

} // verus!

fn main() {
    test_range_set_operations();
}
