// Variation: Vector Operations with Specifications
// From: source/verismo/src/primitives_e/vec.rs (push, remove)
// Demonstrates: Vector operations with formal specifications

use vstd::prelude::*;

verus! {

pub struct VecWrapper<T> {
    pub inner: Vec<T>,
}

impl<T> VecWrapper<T> {
    pub fn new() -> (result: Self)
        ensures
            result@.len() == 0,
    {
        VecWrapper { inner: Vec::new() }
    }

    pub fn push(&mut self, val: T)
        ensures
            self@ == old(self)@.push(val),
    {
        self.inner.push(val);
    }

    pub fn pop(&mut self) -> (result: Option<T>)
        ensures
            match result {
                Some(v) => {
                    &&& old(self)@.len() > 0
                    &&& self@ == old(self)@.subrange(0, old(self)@.len() - 1)
                    &&& v == old(self)@.last()
                },
                None => {
                    &&& old(self)@.len() == 0
                    &&& self@ == old(self)@
                },
            },
    {
        self.inner.pop()
    }

    pub fn len(&self) -> (result: usize)
        ensures
            result == self@.len(),
    {
        self.inner.len()
    }

    pub fn is_empty(&self) -> (result: bool)
        ensures
            result <==> self@.len() == 0,
    {
        self.inner.is_empty()
    }

    pub fn get(&self, index: usize) -> (result: Option<&T>)
        ensures
            match result {
                Some(v) => {
                    &&& index < self@.len()
                    &&& *v == self@[index as int]
                },
                None => index >= self@.len(),
            },
    {
        self.inner.get(index)
    }

    pub fn set(&mut self, index: usize, val: T)
        requires
            index < old(self)@.len(),
        ensures
            self@.len() == old(self)@.len(),
            self@[index as int] == val,
            forall|i: int| 0 <= i < self@.len() && i != index ==> self@[i] == old(self)@[i],
    {
        self.inner[index] = val;
    }

    pub fn clear(&mut self)
        ensures
            self@.len() == 0,
    {
        self.inner.clear();
    }
}

impl<T> View for VecWrapper<T> {
    type V = Seq<T>;

    closed spec fn view(&self) -> Seq<T> {
        self.inner@
    }
}

// Note: Exec function with loop invariants requires careful overflow handling
// pub fn vec_sum(v: &VecWrapper<u64>) -> (result: u64)
//     ensures
//         result == spec_vec_sum(v@),
// {
//     let mut sum: u64 = 0;
//     let mut i: usize = 0;
//     while i < v.len()
//         invariant
//             0 <= i <= v@.len(),
//             sum == spec_vec_sum(v@.subrange(0, i as int)),
//         decreases v@.len() - i,
//     {
//         sum = sum + v.inner[i];
//         i = i + 1;
//     }
//     sum
// }

pub open spec fn spec_vec_sum(s: Seq<u64>) -> u64
    decreases s.len(),
{
    if s.len() == 0 {
        0u64
    } else {
        (spec_vec_sum(s.subrange(0, s.len() - 1)) + s.last()) as u64
    }
}

pub fn vec_contains(v: &VecWrapper<u64>, target: u64) -> (result: bool)
    ensures
        result <==> exists|i: int| 0 <= i < v@.len() && v@[i] == target,
{
    let mut i: usize = 0;
    while i < v.len()
        invariant
            0 <= i <= v@.len(),
            forall|j: int| 0 <= j < i ==> v@[j] != target,
        decreases v@.len() - i,
    {
        if v.inner[i] == target {
            return true;
        }
        i = i + 1;
    }
    false
}

fn test_vector_operations() {
    let mut v = VecWrapper::new();

    v.push(10);
    v.push(20);
    v.push(30);

    let len = v.len();
    let is_empty = v.is_empty();

    let val = v.pop();

    v.set(0, 15);

    let contains = vec_contains(&v, 20);

    v.clear();

    let empty_after = v.is_empty();
}

} // verus!

fn main() {
    test_vector_operations();
}
