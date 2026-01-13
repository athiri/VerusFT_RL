// Variation: Sequence Well-Formedness
// From: source/verismo/src/primitives_e/seq.rs (WellFormed trait)
// Demonstrates: Checking all elements in a sequence satisfy a property

use vstd::prelude::*;

verus! {

pub trait WellFormed {
    spec fn wf(&self) -> bool;
}

impl WellFormed for u64 {
    open spec fn wf(&self) -> bool {
        true
    }
}

impl WellFormed for i64 {
    open spec fn wf(&self) -> bool {
        true
    }
}

pub struct BoundedValue {
    pub value: u64,
    pub max: u64,
}

impl WellFormed for BoundedValue {
    open spec fn wf(&self) -> bool {
        self.value <= self.max
    }
}

impl BoundedValue {
    pub fn new(value: u64, max: u64) -> (result: Self)
        requires
            value <= max,
        ensures
            result.wf(),
            result.value == value,
            result.max == max,
    {
        BoundedValue { value, max }
    }

    pub fn is_valid(&self) -> (result: bool)
        ensures
            result <==> self.wf(),
    {
        self.value <= self.max
    }
}

pub open spec fn seq_wf<T: WellFormed>(s: Seq<T>) -> bool {
    forall|i: int| 0 <= i < s.len() ==> #[trigger] s[i].wf()
}

pub open spec fn all_elements_valid(s: Seq<BoundedValue>) -> bool {
    forall|i: int| 0 <= i < s.len() ==> s[i].wf()
}

pub proof fn lemma_empty_seq_wf<T: WellFormed>()
    ensures
        seq_wf(Seq::<T>::empty()),
{
}

pub proof fn lemma_seq_wf_subrange<T: WellFormed>(s: Seq<T>, start: int, end: int)
    requires
        seq_wf(s),
        0 <= start <= end <= s.len(),
    ensures
        seq_wf(s.subrange(start, end)),
{
}

pub proof fn lemma_seq_wf_push<T: WellFormed>(s: Seq<T>, val: T)
    requires
        seq_wf(s),
        val.wf(),
    ensures
        seq_wf(s.push(val)),
{
}

fn test_sequence_wellformedness() {
    let val1 = BoundedValue::new(10, 100);
    let val2 = BoundedValue::new(50, 100);

    proof {
        let s = Seq::new(2, |i: int| if i == 0 { val1 } else { val2 });
        lemma_empty_seq_wf::<BoundedValue>();
        let empty = Seq::<BoundedValue>::empty();
        assert(all_elements_valid(empty));
    }
}

} // verus!

fn main() {
    test_sequence_wellformedness();
}
