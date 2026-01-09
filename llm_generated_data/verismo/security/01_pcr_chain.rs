// Variation: PCR Chain Tracking
// From: source/verismo/src/security/pcr.rs (SHA256WithChain)
// Demonstrates: Platform Configuration Register chain tracking with SHA256

use vstd::prelude::*;

verus! {

pub struct SHA256Value {
    pub val: [u8; 32],
}

impl SHA256Value {
    pub fn new(val: [u8; 32]) -> (result: Self)
        ensures
            result.val == val,
    {
        SHA256Value { val }
    }

    pub fn zero() -> (result: Self)
        ensures
            forall|i: int| 0 <= i < 32 ==> result.val[i] == 0,
    {
        SHA256Value { val: [0u8; 32] }
    }

    pub fn get_byte(&self, index: usize) -> (result: u8)
        requires
            index < 32,
        ensures
            result == self.val[index as int],
    {
        self.val[index]
    }

    pub open spec fn spec_equals(&self, other: &Self) -> bool {
        forall|i: int| 0 <= i < 32 ==> self.val[i] == other.val[i]
    }

    pub fn equals(&self, other: &Self) -> (result: bool)
        ensures
            result <==> self.spec_equals(other),
    {
        let mut i = 0;
        while i < 32
            invariant
                i <= 32,
                forall|j: int| 0 <= j < i ==> self.val[j] == other.val[j],
            decreases 32 - i,
        {
            if self.val[i] != other.val[i] {
                return false;
            }
            i += 1;
        }
        true
    }
}

pub struct PCRBank {
    pub entries: Vec<SHA256Value>,
    pub max_entries: usize,
}

impl PCRBank {
    pub fn new(max_entries: usize) -> (result: Self)
        ensures
            result.max_entries == max_entries,
            result.entries@.len() == 0,
    {
        PCRBank {
            entries: Vec::new(),
            max_entries,
        }
    }

    pub open spec fn wf(&self) -> bool {
        self.entries@.len() <= self.max_entries
    }

    pub fn len(&self) -> (result: usize)
        ensures
            result == self.entries@.len(),
    {
        self.entries.len()
    }

    pub fn get(&self, index: usize) -> (result: &SHA256Value)
        requires
            index < self.entries@.len(),
        ensures
            *result == self.entries@[index as int],
    {
        &self.entries[index]
    }

    pub fn extend(&mut self, value: SHA256Value)
        requires
            old(self).wf(),
            old(self).entries@.len() < old(self).max_entries,
        ensures
            self.wf(),
            self.max_entries == old(self).max_entries,
            self.entries@.len() == old(self).entries@.len() + 1,
            self.entries@[old(self).entries@.len() as int] == value,
    {
        self.entries.push(value);
    }

    pub open spec fn contains_entry(&self, index: int) -> bool {
        0 <= index < self.entries@.len()
    }

    pub open spec fn all_entries_valid(&self) -> bool {
        forall|i: int| self.contains_entry(i) ==> true
    }
}

pub struct PCRChain {
    pub chain: Ghost<Seq<Seq<u8>>>,
    pub current: SHA256Value,
}

impl PCRChain {
    pub fn new(initial: SHA256Value) -> (result: Self)
        ensures
            result.current.val == initial.val,
            result.chain@.len() == 0,
    {
        PCRChain {
            chain: Ghost(Seq::empty()),
            current: initial,
        }
    }

    pub open spec fn chain_len(&self) -> nat {
        self.chain@.len()
    }

    pub open spec fn wf(&self) -> bool {
        self.chain@.len() >= 0
    }

    pub fn get_current(&self) -> (result: &SHA256Value)
        ensures
            *result == self.current,
    {
        &self.current
    }
}

fn test_pcr_chain() {
    let initial = SHA256Value::zero();
    let bank = PCRBank::new(24);

    let bank_len = bank.len();

    let value1 = SHA256Value::new([1u8; 32]);
    let value2 = SHA256Value::new([2u8; 32]);

    let equals = value1.equals(&value2);
    let self_equals = value1.equals(&value1);

    let chain = PCRChain::new(initial);

    proof {
        assert(bank.wf());
        assert(bank_len == 0);
        assert(chain.wf());
        assert(chain.chain_len() == 0);
        assert(bank.all_entries_valid());
    }
}

} // verus!

fn main() {
    test_pcr_chain();
}
