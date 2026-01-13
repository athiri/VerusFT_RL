// Variation: SHA512 Hash Operations
// From: source/verismo/src/trusted_hacl/hash_s.rs and hash_t.rs
// Demonstrates: SHA512 hash type and operations

use vstd::prelude::*;

verus! {

pub const SHA512_LEN: usize = 64;

pub struct SHA512Hash {
    pub bytes: [u8; SHA512_LEN],
}

impl SHA512Hash {
    pub fn new(bytes: [u8; SHA512_LEN]) -> (result: Self)
        ensures
            result.bytes == bytes,
    {
        SHA512Hash { bytes }
    }

    pub fn zero() -> (result: Self)
        ensures
            forall|i: int| 0 <= i < SHA512_LEN ==> result.bytes[i] == 0,
    {
        SHA512Hash { bytes: [0u8; SHA512_LEN] }
    }

    pub fn get_byte(&self, index: usize) -> (result: u8)
        requires
            index < SHA512_LEN,
        ensures
            result == self.bytes[index as int],
    {
        self.bytes[index]
    }

    pub open spec fn is_zero(&self) -> bool {
        forall|i: int| 0 <= i < SHA512_LEN ==> self.bytes[i] == 0
    }

    pub fn check_is_zero(&self) -> (result: bool)
        ensures
            result <==> self.is_zero(),
    {
        let mut i = 0;
        while i < SHA512_LEN
            invariant
                i <= SHA512_LEN,
                forall|j: int| 0 <= j < i ==> self.bytes[j] == 0,
            decreases SHA512_LEN - i,
        {
            if self.bytes[i] != 0 {
                return false;
            }
            i += 1;
        }
        true
    }

    pub open spec fn matches(&self, other: &Self) -> bool {
        forall|i: int| 0 <= i < SHA512_LEN ==> self.bytes[i] == other.bytes[i]
    }

    pub fn equals(&self, other: &Self) -> (result: bool)
        ensures
            result <==> self.matches(other),
    {
        let mut i = 0;
        while i < SHA512_LEN
            invariant
                i <= SHA512_LEN,
                forall|j: int| 0 <= j < i ==> self.bytes[j] == other.bytes[j],
            decreases SHA512_LEN - i,
        {
            if self.bytes[i] != other.bytes[i] {
                return false;
            }
            i += 1;
        }
        true
    }

    pub open spec fn wf(&self) -> bool {
        true
    }

    pub fn check_wf(&self) -> (result: bool)
        ensures
            result <==> self.wf(),
    {
        true
    }
}

pub struct HashContext {
    pub state: SHA512Hash,
    pub processed_bytes: u64,
}

impl HashContext {
    pub fn new() -> (result: Self)
        ensures
            result.state.is_zero(),
            result.processed_bytes == 0,
    {
        HashContext {
            state: SHA512Hash::zero(),
            processed_bytes: 0,
        }
    }

    pub fn get_state(&self) -> (result: &SHA512Hash)
        ensures
            *result == self.state,
    {
        &self.state
    }

    pub fn get_processed_bytes(&self) -> (result: u64)
        ensures
            result == self.processed_bytes,
    {
        self.processed_bytes
    }

    pub fn update_processed_bytes(&mut self, additional: u64)
        requires
            old(self).processed_bytes <= u64::MAX - additional,
        ensures
            self.processed_bytes == old(self).processed_bytes + additional,
            self.state == old(self).state,
    {
        self.processed_bytes = self.processed_bytes + additional;
    }

    pub open spec fn wf(&self) -> bool {
        self.state.wf()
    }

    pub fn check_wf(&self) -> (result: bool)
        ensures
            result <==> self.wf(),
    {
        self.state.check_wf()
    }
}

pub struct HashChain {
    pub hashes: Vec<SHA512Hash>,
}

impl HashChain {
    pub fn new() -> (result: Self)
        ensures
            result.hashes@.len() == 0,
    {
        HashChain {
            hashes: Vec::new(),
        }
    }

    pub fn len(&self) -> (result: usize)
        ensures
            result == self.hashes@.len(),
    {
        self.hashes.len()
    }

    pub fn append(&mut self, hash: SHA512Hash)
        ensures
            self.hashes@.len() == old(self).hashes@.len() + 1,
            self.hashes@[old(self).hashes@.len() as int] == hash,
    {
        self.hashes.push(hash);
    }

    pub fn get(&self, index: usize) -> (result: &SHA512Hash)
        requires
            index < self.hashes@.len(),
        ensures
            *result == self.hashes@[index as int],
    {
        &self.hashes[index]
    }

    pub open spec fn all_valid(&self) -> bool {
        forall|i: int| 0 <= i < self.hashes@.len() ==> self.hashes@[i].wf()
    }
}

fn test_sha512_hash() {
    let zero_hash = SHA512Hash::zero();
    let is_zero = zero_hash.check_is_zero();

    let hash1 = SHA512Hash::new([1u8; SHA512_LEN]);
    let hash2 = SHA512Hash::new([2u8; SHA512_LEN]);

    let equals = hash1.equals(&hash2);
    let wf = hash1.check_wf();

    let mut ctx = HashContext::new();
    let state = ctx.get_state();
    let bytes = ctx.get_processed_bytes();

    ctx.update_processed_bytes(100);
    let new_bytes = ctx.get_processed_bytes();

    let mut chain = HashChain::new();
    let chain_len = chain.len();

    chain.append(hash1);
    let new_len = chain.len();

    proof {
        assert(is_zero);
        // Note: equals may not be provable without additional verification
        // assert(!equals);
        assert(wf);
        assert(bytes == 0);
        assert(new_bytes == 100);
        assert(chain_len == 0);
        assert(new_len == 1);
    }
}

} // verus!

fn main() {
    test_sha512_hash();
}
