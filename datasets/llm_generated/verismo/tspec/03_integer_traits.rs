// Variation: Integer Traits and Operations
// From: source/verismo/src/tspec/integer.rs
// Demonstrates: IntValue and IntOrd traits for type-safe integer operations

use vstd::prelude::*;

verus! {

pub trait IntValue {
    spec fn as_int(&self) -> int;
}

impl IntValue for u8 {
    open spec fn as_int(&self) -> int {
        *self as int
    }
}

impl IntValue for u16 {
    open spec fn as_int(&self) -> int {
        *self as int
    }
}

impl IntValue for u32 {
    open spec fn as_int(&self) -> int {
        *self as int
    }
}

impl IntValue for u64 {
    open spec fn as_int(&self) -> int {
        *self as int
    }
}

impl IntValue for usize {
    open spec fn as_int(&self) -> int {
        *self as int
    }
}

impl IntValue for i8 {
    open spec fn as_int(&self) -> int {
        *self as int
    }
}

impl IntValue for i16 {
    open spec fn as_int(&self) -> int {
        *self as int
    }
}

impl IntValue for i32 {
    open spec fn as_int(&self) -> int {
        *self as int
    }
}

impl IntValue for i64 {
    open spec fn as_int(&self) -> int {
        *self as int
    }
}

pub trait IntOrd {
    spec fn le(&self, other: &Self) -> bool;
    spec fn lt(&self, other: &Self) -> bool;
    spec fn ge(&self, other: &Self) -> bool;
    spec fn gt(&self, other: &Self) -> bool;
}

impl<T: IntValue> IntOrd for T {
    open spec fn le(&self, other: &Self) -> bool {
        self.as_int() <= other.as_int()
    }

    open spec fn lt(&self, other: &Self) -> bool {
        self.as_int() < other.as_int()
    }

    open spec fn ge(&self, other: &Self) -> bool {
        self.as_int() >= other.as_int()
    }

    open spec fn gt(&self, other: &Self) -> bool {
        self.as_int() > other.as_int()
    }
}

pub struct Range<T> {
    pub start: T,
    pub end: T,
}

impl<T: IntValue> Range<T> {
    pub fn new(start: T, end: T) -> (result: Self)
        ensures
            result.start.as_int() == start.as_int(),
            result.end.as_int() == end.as_int(),
    {
        Range { start, end }
    }

    pub open spec fn is_empty(&self) -> bool {
        self.start.as_int() >= self.end.as_int()
    }

    pub open spec fn len(&self) -> int {
        if self.is_empty() {
            0
        } else {
            self.end.as_int() - self.start.as_int()
        }
    }

    pub open spec fn contains(&self, val: T) -> bool {
        self.start.as_int() <= val.as_int() < self.end.as_int()
    }

    pub open spec fn overlaps(&self, other: &Self) -> bool {
        !(self.end.as_int() <= other.start.as_int() || other.end.as_int() <= self.start.as_int())
    }

    pub open spec fn wf(&self) -> bool {
        self.start.as_int() <= self.end.as_int()
    }
}

impl<T: IntValue + Copy> Range<T> {
    pub fn get_start(&self) -> (result: T)
        ensures
            result.as_int() == self.start.as_int(),
    {
        self.start
    }

    pub fn get_end(&self) -> (result: T)
        ensures
            result.as_int() == self.end.as_int(),
    {
        self.end
    }
}

pub struct Bounds<T> {
    pub min: T,
    pub max: T,
}

impl<T: IntValue> Bounds<T> {
    pub fn new(min: T, max: T) -> (result: Self)
        ensures
            result.min.as_int() == min.as_int(),
            result.max.as_int() == max.as_int(),
    {
        Bounds { min, max }
    }

    pub open spec fn in_bounds(&self, val: T) -> bool {
        self.min.as_int() <= val.as_int() <= self.max.as_int()
    }

    pub open spec fn wf(&self) -> bool {
        self.min.as_int() <= self.max.as_int()
    }

    pub open spec fn range_len(&self) -> int {
        self.max.as_int() - self.min.as_int() + 1
    }
}

impl<T: IntValue + Copy> Bounds<T> {
    pub fn get_min(&self) -> (result: T)
        ensures
            result.as_int() == self.min.as_int(),
    {
        self.min
    }

    pub fn get_max(&self) -> (result: T)
        ensures
            result.as_int() == self.max.as_int(),
    {
        self.max
    }
}

pub struct IndexedValue<T> {
    pub index: usize,
    pub value: T,
}

impl<T> IndexedValue<T> {
    pub fn new(index: usize, value: T) -> (result: Self)
        ensures
            result.index == index,
            result.value == value,
    {
        IndexedValue { index, value }
    }

    pub fn get_index(&self) -> (result: usize)
        ensures
            result == self.index,
    {
        self.index
    }

    pub fn get_value(&self) -> (result: &T)
        ensures
            *result == self.value,
    {
        &self.value
    }
}

impl<T: IntValue> IndexedValue<T> {
    pub open spec fn wf(&self, max_index: usize) -> bool {
        self.index < max_index
    }

    pub fn check_wf(&self, max_index: usize) -> (result: bool)
        ensures
            result <==> self.wf(max_index),
    {
        self.index < max_index
    }
}

fn test_integer_traits() {
    let u8_val: u8 = 42;
    let u16_val: u16 = 1000;
    let u32_val: u32 = 50000;

    let range = Range::new(10u64, 100u64);
    let start = range.get_start();
    let end = range.get_end();

    let bounds = Bounds::new(0u32, 255u32);
    let min = bounds.get_min();
    let max = bounds.get_max();

    let indexed = IndexedValue::new(5, 100u64);
    let index = indexed.get_index();
    let value = indexed.get_value();
    let indexed_wf = indexed.check_wf(10);

    proof {
        assert(u8_val.as_int() == 42);
        assert(u16_val.as_int() == 1000);
        assert(u32_val.as_int() == 50000);
        assert(!range.is_empty());
        assert(range.contains(50u64));
        assert(range.wf());
        assert(start == 10);
        assert(end == 100);
        assert(bounds.in_bounds(128u32));
        assert(bounds.wf());
        assert(min == 0);
        assert(max == 255);
        assert(index == 5);
        assert(*value == 100);
        assert(indexed_wf);
        assert(range.len() == 90);
        assert(bounds.range_len() == 256);
    }
}

} // verus!

fn main() {
    test_integer_traits();
}
