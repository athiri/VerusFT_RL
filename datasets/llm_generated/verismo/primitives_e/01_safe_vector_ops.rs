// Variation: Safe Vector Operations
// From: source/verismo/src/primitives_e/vec.rs
// Demonstrates: Safe push/remove with specifications

use vstd::prelude::*;

verus! {

pub struct SafeVec<T> {
    pub data: Vec<T>,
}

impl<T> SafeVec<T> {
    pub fn new() -> (result: Self)
        ensures
            result.data@.len() == 0,
    {
        SafeVec { data: Vec::new() }
    }

    pub fn len(&self) -> (result: usize)
        ensures
            result == self.data@.len(),
    {
        self.data.len()
    }

    pub fn is_empty(&self) -> (result: bool)
        ensures
            result <==> self.data@.len() == 0,
    {
        self.data.len() == 0
    }

    pub fn push(&mut self, value: T)
        ensures
            self.data@ == old(self).data@.push(value),
    {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> (result: Option<T>)
        ensures
            match result {
                Some(v) => {
                    &&& old(self).data@.len() > 0
                    &&& self.data@ == old(self).data@.drop_last()
                    &&& v == old(self).data@.last()
                },
                None => {
                    &&& old(self).data@.len() == 0
                    &&& self.data@ == old(self).data@
                },
            },
    {
        self.data.pop()
    }

    pub fn get(&self, index: usize) -> (result: Option<&T>)
        ensures
            match result {
                Some(v) => index < self.data@.len() && v == self.data@[index as int],
                None => index >= self.data@.len(),
            },
    {
        if index < self.data.len() {
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn set(&mut self, index: usize, value: T) -> (result: bool)
        ensures
            result ==> {
                &&& index < old(self).data@.len()
                &&& self.data@ == old(self).data@.update(index as int, value)
            },
            !result ==> {
                &&& index >= old(self).data@.len()
                &&& self.data@ == old(self).data@
            },
    {
        if index < self.data.len() {
            self.data.set(index, value);
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self)
        ensures
            self.data@.len() == 0,
    {
        self.data.clear();
    }
}

fn test_safe_vec() {
    let mut vec = SafeVec::<i32>::new();
    let empty = vec.is_empty();
    assert(empty);

    vec.push(10);
    vec.push(20);
    vec.push(30);

    let len = vec.len();
    assert(len == 3);

    let val = vec.get(1);
    assert(val.is_some());
    assert(*val.unwrap() == 20);

    let out_of_bounds = vec.get(10);
    assert(out_of_bounds.is_none());

    let success = vec.set(1, 25);
    assert(success);

    let fail = vec.set(10, 100);
    assert(!fail);

    let popped = vec.pop();
    assert(popped.is_some());
    assert(popped.unwrap() == 30);

    vec.clear();
    let empty_again = vec.is_empty();
    assert(empty_again);
}

} // verus!

fn main() {
    test_safe_vec();
}
