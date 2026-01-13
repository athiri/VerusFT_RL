// Variation: Array Specifications
// From: source/verismo/src/tspec_e/array/array_e.rs
// Demonstrates: Array index and update specifications

use vstd::prelude::*;

verus! {

pub struct ArrayBounds {
    pub index: usize,
    pub length: usize,
}

impl ArrayBounds {
    pub fn new(index: usize, length: usize) -> (result: Self)
        ensures
            result.index == index,
            result.length == length,
    {
        ArrayBounds { index, length }
    }

    pub open spec fn in_bounds(&self) -> bool {
        self.index < self.length
    }

    pub open spec fn wf(&self) -> bool {
        self.length > 0
    }

    pub fn get_index(&self) -> (result: usize)
        ensures
            result == self.index,
    {
        self.index
    }

    pub fn get_length(&self) -> (result: usize)
        ensures
            result == self.length,
    {
        self.length
    }
}

pub struct VecWrapper<T> {
    pub data: Vec<T>,
}

impl<T> VecWrapper<T> {
    pub fn new(data: Vec<T>) -> (result: Self)
        ensures
            result.data@ == data@,
    {
        VecWrapper { data }
    }

    pub fn empty() -> (result: Self)
        ensures
            result.data@.len() == 0,
    {
        VecWrapper { data: Vec::new() }
    }

    pub fn len(&self) -> (result: usize)
        ensures
            result == self.data@.len(),
    {
        self.data.len()
    }

    pub open spec fn spec_index(&self, i: int) -> T {
        self.data@[i]
    }

    pub open spec fn wf(&self) -> bool {
        true
    }
}

impl<T: Copy> VecWrapper<T> {
    pub fn index(&self, i: usize) -> (result: T)
        requires
            i < self.data@.len(),
        ensures
            result == self.spec_index(i as int),
    {
        self.data[i]
    }
}

pub struct IndexedAccess<T> {
    pub vec: Vec<T>,
    pub last_index: Option<usize>,
}

impl<T> IndexedAccess<T> {
    pub fn new(vec: Vec<T>) -> (result: Self)
        ensures
            result.vec@ == vec@,
            matches!(result.last_index, None),
    {
        IndexedAccess {
            vec,
            last_index: None,
        }
    }

    pub fn len(&self) -> (result: usize)
        ensures
            result == self.vec@.len(),
    {
        self.vec.len()
    }

    pub open spec fn has_last_index(&self) -> bool {
        matches!(self.last_index, Some(_))
    }

    pub open spec fn wf(&self) -> bool {
        match self.last_index {
            Some(i) => i < self.vec@.len(),
            None => true,
        }
    }

    pub fn get_last_index(&self) -> (result: Option<usize>)
        ensures
            result == self.last_index,
    {
        self.last_index
    }
}

impl<T: Copy> IndexedAccess<T> {
    pub fn get(&mut self, index: usize) -> (result: T)
        requires
            index < old(self).vec@.len(),
        ensures
            result == old(self).vec@[index as int],
            self.vec@ == old(self).vec@,
            self.last_index == Some(index),
    {
        self.last_index = Some(index);
        self.vec[index]
    }
}

pub struct SliceBounds {
    pub start: usize,
    pub end: usize,
    pub len: usize,
}

impl SliceBounds {
    pub fn new(start: usize, end: usize, len: usize) -> (result: Self)
        ensures
            result.start == start,
            result.end == end,
            result.len == len,
    {
        SliceBounds { start, end, len }
    }

    pub open spec fn wf(&self) -> bool {
        &&& self.start <= self.end
        &&& self.end <= self.len
    }

    pub open spec fn slice_len(&self) -> int {
        self.end as int - self.start as int
    }

    pub open spec fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn get_start(&self) -> (result: usize)
        ensures
            result == self.start,
    {
        self.start
    }

    pub fn get_end(&self) -> (result: usize)
        ensures
            result == self.end,
    {
        self.end
    }

    pub fn get_len(&self) -> (result: usize)
        ensures
            result == self.len,
    {
        self.len
    }
}

pub struct UpdateOperation {
    pub index: usize,
    pub old_value: u64,
    pub new_value: u64,
}

impl UpdateOperation {
    pub fn new(index: usize, old_value: u64, new_value: u64) -> (result: Self)
        ensures
            result.index == index,
            result.old_value == old_value,
            result.new_value == new_value,
    {
        UpdateOperation {
            index,
            old_value,
            new_value,
        }
    }

    pub open spec fn changed(&self) -> bool {
        self.old_value != self.new_value
    }

    pub fn get_index(&self) -> (result: usize)
        ensures
            result == self.index,
    {
        self.index
    }

    pub fn get_old_value(&self) -> (result: u64)
        ensures
            result == self.old_value,
    {
        self.old_value
    }

    pub fn get_new_value(&self) -> (result: u64)
        ensures
            result == self.new_value,
    {
        self.new_value
    }

    pub open spec fn wf(&self) -> bool {
        true
    }
}

pub struct ConstantIndex {
    pub value: usize,
}

impl ConstantIndex {
    pub fn new(value: usize) -> (result: Self)
        ensures
            result.value == value,
    {
        ConstantIndex { value }
    }

    pub open spec fn is_valid_for(&self, len: usize) -> bool {
        self.value < len
    }

    pub fn get_value(&self) -> (result: usize)
        ensures
            result == self.value,
    {
        self.value
    }

    pub open spec fn wf(&self) -> bool {
        true
    }
}

fn test_array_specs() {
    let bounds = ArrayBounds::new(5, 10);
    let index = bounds.get_index();
    let length = bounds.get_length();

    let vec: VecWrapper<u64> = VecWrapper::new(Vec::new());
    let len = vec.len();

    let empty_vec: VecWrapper<u64> = VecWrapper::empty();
    let empty_len = empty_vec.len();

    let mut indexed: IndexedAccess<u64> = IndexedAccess::new(Vec::new());
    let indexed_len = indexed.len();
    let last_index = indexed.get_last_index();

    let slice_bounds = SliceBounds::new(2, 8, 10);
    let start = slice_bounds.get_start();
    let end = slice_bounds.get_end();
    let slice_len = slice_bounds.get_len();

    let update = UpdateOperation::new(3, 100, 200);
    let upd_index = update.get_index();
    let old_val = update.get_old_value();
    let new_val = update.get_new_value();

    let const_idx = ConstantIndex::new(7);
    let idx_val = const_idx.get_value();

    proof {
        assert(index == 5);
        assert(length == 10);
        assert(bounds.in_bounds());
        assert(bounds.wf());
        assert(len == 0);
        assert(vec.wf());
        assert(empty_len == 0);
        assert(empty_vec.wf());
        assert(indexed_len == 0);
        assert(matches!(last_index, None));
        assert(!indexed.has_last_index());
        assert(indexed.wf());
        assert(start == 2);
        assert(end == 8);
        assert(slice_len == 10);
        assert(slice_bounds.slice_len() == 6);
        assert(!slice_bounds.is_empty());
        assert(slice_bounds.wf());
        assert(upd_index == 3);
        assert(old_val == 100);
        assert(new_val == 200);
        assert(update.changed());
        assert(update.wf());
        assert(idx_val == 7);
        assert(const_idx.is_valid_for(10));
        assert(!const_idx.is_valid_for(5));
        assert(const_idx.wf());
    }
}

} // verus!

fn main() {
    test_array_specs();
}
