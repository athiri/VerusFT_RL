use vstd::prelude::*;

verus! {

pub struct TestData<A> {
    pub value: A,
    pub size: nat,
    pub shrinks: Seq<A>,
}

pub open spec fn map_test_data<A, B>(data: TestData<A>, f: spec_fn(A) -> B) -> TestData<B>
    where A: std::marker::Copy
{
    TestData {
        value: f(data.value),
        size: data.size,
        shrinks: data.shrinks.map(|_i: int, a: A| f(a)),
    }
}


pub proof fn map_preserves_size<A, B>(data: TestData<A>, f: spec_fn(A) -> B)
    where A: std::marker::Copy
    ensures map_test_data(data, f).size == data.size
{
}

} // verus!