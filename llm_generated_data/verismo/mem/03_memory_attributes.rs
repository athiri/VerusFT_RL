// Variation: Memory Attributes
// From: source/verismo/src/mem/rawmem_p.rs (memory attribute concepts)
// Demonstrates: Memory region attributes and states

use vstd::prelude::*;

verus! {

pub enum MemoryState {
    Uninitialized,
    Initialized,
    Allocated,
    Free,
}

pub struct MemoryAttributes {
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
    pub state: MemoryState,
}

impl MemoryAttributes {
    pub fn new_default() -> (result: Self)
        ensures
            result.readable,
            result.writable,
            !result.executable,
    {
        MemoryAttributes {
            readable: true,
            writable: true,
            executable: false,
            state: MemoryState::Uninitialized,
        }
    }

    pub fn new_initialized() -> (result: Self)
        ensures
            result.readable,
            result.writable,
    {
        MemoryAttributes {
            readable: true,
            writable: true,
            executable: false,
            state: MemoryState::Initialized,
        }
    }

    pub fn new_readonly() -> (result: Self)
        ensures
            result.readable,
            !result.writable,
    {
        MemoryAttributes {
            readable: true,
            writable: false,
            executable: false,
            state: MemoryState::Initialized,
        }
    }

    pub fn is_readable(&self) -> (result: bool)
        ensures
            result <==> self.readable,
    {
        self.readable
    }

    pub fn is_writable(&self) -> (result: bool)
        ensures
            result <==> self.writable,
    {
        self.writable
    }

    pub fn is_executable(&self) -> (result: bool)
        ensures
            result <==> self.executable,
    {
        self.executable
    }

    pub fn is_read_write(&self) -> (result: bool)
        ensures
            result <==> (self.readable && self.writable),
    {
        self.readable && self.writable
    }

    pub fn is_read_only(&self) -> (result: bool)
        ensures
            result <==> (self.readable && !self.writable),
    {
        self.readable && !self.writable
    }

    pub fn set_writable(&mut self)
        ensures
            self.writable,
            self.readable == old(self).readable,
            self.executable == old(self).executable,
    {
        self.writable = true;
    }

    pub fn set_readonly(&mut self)
        ensures
            !self.writable,
            self.readable == old(self).readable,
            self.executable == old(self).executable,
    {
        self.writable = false;
    }

    pub fn matches(&self, other: &MemoryAttributes) -> (result: bool)
        ensures
            result <==> (self.readable == other.readable
                      && self.writable == other.writable
                      && self.executable == other.executable),
    {
        self.readable == other.readable
            && self.writable == other.writable
            && self.executable == other.executable
    }
}

fn test_memory_attributes() {
    let attrs1 = MemoryAttributes::new_default();
    let attrs2 = MemoryAttributes::new_initialized();
    let attrs3 = MemoryAttributes::new_readonly();

    let readable1 = attrs1.is_readable();
    let writable1 = attrs1.is_writable();
    let executable1 = attrs1.is_executable();

    let rw1 = attrs1.is_read_write();
    let ro1 = attrs1.is_read_only();
    let ro3 = attrs3.is_read_only();

    let mut attrs4 = MemoryAttributes::new_default();
    attrs4.set_readonly();
    let ro4 = attrs4.is_read_only();

    attrs4.set_writable();
    let rw4 = attrs4.is_read_write();

    let match12 = attrs1.matches(&attrs2);
    let match13 = attrs1.matches(&attrs3);
}

} // verus!

fn main() {
    test_memory_attributes();
}
