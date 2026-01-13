// Variation: Memory Permissions
// From: source/verismo/src/security/mem.rs (OSMemPermSpec, OSMemPerm)
// Demonstrates: Memory permission bits for read/write/execute control

use vstd::prelude::*;

verus! {

pub struct MemPermBits {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub user: bool,
}

impl MemPermBits {
    pub fn new(read: bool, write: bool, execute: bool, user: bool) -> (result: Self)
        ensures
            result.read == read,
            result.write == write,
            result.execute == execute,
            result.user == user,
    {
        MemPermBits { read, write, execute, user }
    }

    pub fn empty() -> (result: Self)
        ensures
            !result.read,
            !result.write,
            !result.execute,
            !result.user,
    {
        MemPermBits { read: false, write: false, execute: false, user: false }
    }

    pub fn readonly() -> (result: Self)
        ensures
            result.read,
            !result.write,
            !result.execute,
            !result.user,
    {
        MemPermBits { read: true, write: false, execute: false, user: false }
    }

    pub fn readwrite() -> (result: Self)
        ensures
            result.read,
            result.write,
            !result.execute,
            !result.user,
    {
        MemPermBits { read: true, write: true, execute: false, user: false }
    }

    pub fn execute_only() -> (result: Self)
        ensures
            !result.read,
            !result.write,
            result.execute,
            !result.user,
    {
        MemPermBits { read: false, write: false, execute: true, user: false }
    }

    pub fn full_access() -> (result: Self)
        ensures
            result.read,
            result.write,
            result.execute,
            result.user,
    {
        MemPermBits { read: true, write: true, execute: true, user: true }
    }

    pub open spec fn is_super_of(&self, other: &Self) -> bool {
        &&& (self.read ==> other.read)
        &&& (self.write ==> other.write)
        &&& (self.execute ==> other.execute)
        &&& (self.user ==> other.user)
    }

    pub fn check_is_super_of(&self, other: &Self) -> (result: bool)
        ensures
            result <==> self.is_super_of(other),
    {
        (!self.read || other.read) &&
        (!self.write || other.write) &&
        (!self.execute || other.execute) &&
        (!self.user || other.user)
    }

    pub fn can_read(&self) -> (result: bool)
        ensures
            result == self.read,
    {
        self.read
    }

    pub fn can_write(&self) -> (result: bool)
        ensures
            result == self.write,
    {
        self.write
    }

    pub fn can_execute(&self) -> (result: bool)
        ensures
            result == self.execute,
    {
        self.execute
    }

    pub fn is_user_accessible(&self) -> (result: bool)
        ensures
            result == self.user,
    {
        self.user
    }

    pub open spec fn allows_operation(&self, read: bool, write: bool, exec: bool) -> bool {
        &&& (read ==> self.read)
        &&& (write ==> self.write)
        &&& (exec ==> self.execute)
    }

    pub fn check_operation(&self, read: bool, write: bool, exec: bool) -> (result: bool)
        ensures
            result <==> self.allows_operation(read, write, exec),
    {
        (!read || self.read) &&
        (!write || self.write) &&
        (!exec || self.execute)
    }
}

pub struct MemPermU8 {
    pub value: u8,
}

impl MemPermU8 {
    pub fn new(value: u8) -> (result: Self)
        ensures
            result.value == value,
    {
        MemPermU8 { value }
    }

    pub fn empty() -> (result: Self)
        ensures
            result.value == 0,
    {
        MemPermU8 { value: 0 }
    }

    pub fn from_bits(read: bool, write: bool, execute: bool, user: bool) -> (result: Self)
    {
        let mut val = 0u8;
        if read { val = val | 0x01; }
        if write { val = val | 0x02; }
        if execute { val = val | 0x04; }
        if user { val = val | 0x08; }
        MemPermU8 { value: val }
    }

    pub fn get_read(&self) -> (result: bool)
    {
        (self.value & 0x01) != 0
    }

    pub fn get_write(&self) -> (result: bool)
    {
        (self.value & 0x02) != 0
    }

    pub fn get_execute(&self) -> (result: bool)
    {
        (self.value & 0x04) != 0
    }

    pub fn get_user(&self) -> (result: bool)
    {
        (self.value & 0x08) != 0
    }

    pub fn is_super_of(&self, other: &Self) -> (result: bool)
    {
        (self.value & other.value) == other.value
    }
}

pub struct MemRegion {
    pub start_addr: usize,
    pub size: usize,
    pub perms: MemPermBits,
}

impl MemRegion {
    pub fn new(start_addr: usize, size: usize, perms: MemPermBits) -> (result: Self)
        ensures
            result.start_addr == start_addr,
            result.size == size,
            result.perms.read == perms.read,
    {
        MemRegion { start_addr, size, perms }
    }

    pub open spec fn contains_addr(&self, addr: usize) -> bool {
        self.start_addr <= addr < self.start_addr + self.size
    }

    pub fn check_contains(&self, addr: usize) -> (result: bool)
        requires
            self.start_addr <= usize::MAX - self.size,
        ensures
            result <==> self.contains_addr(addr),
    {
        self.start_addr <= addr && addr < self.start_addr + self.size
    }

    pub open spec fn overlaps(&self, other: &Self) -> bool {
        !(self.start_addr + self.size <= other.start_addr
          || other.start_addr + other.size <= self.start_addr)
    }

    pub fn check_overlaps(&self, other: &Self) -> (result: bool)
        requires
            self.start_addr <= usize::MAX - self.size,
            other.start_addr <= usize::MAX - other.size,
        ensures
            result <==> self.overlaps(other),
    {
        !(self.start_addr + self.size <= other.start_addr
          || other.start_addr + other.size <= self.start_addr)
    }
}

fn test_memory_permissions() {
    let empty = MemPermBits::empty();
    let ro = MemPermBits::readonly();
    let rw = MemPermBits::readwrite();
    let full = MemPermBits::full_access();

    let empty_can_read = empty.can_read();
    let ro_can_read = ro.can_read();
    let ro_can_write = ro.can_write();
    let rw_can_write = rw.can_write();

    let is_super = full.check_is_super_of(&rw);
    let can_exec = full.check_operation(false, false, true);

    let perm_u8 = MemPermU8::from_bits(true, true, false, false);
    let read = perm_u8.get_read();
    let write = perm_u8.get_write();

    let region1 = MemRegion::new(0x1000, 0x1000, rw);
    let region2 = MemRegion::new(0x2000, 0x1000, ro);

    let contains = region1.check_contains(0x1500);
    let overlaps = region1.check_overlaps(&region2);

    proof {
        assert(!overlaps);
        assert(!empty_can_read);
        assert(ro_can_read);
        assert(!ro_can_write);
    }
}

} // verus!

fn main() {
    test_memory_permissions();
}
