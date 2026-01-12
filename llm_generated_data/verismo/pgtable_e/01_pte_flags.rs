// Variation: Page Table Entry Flags
// From: source/verismo/src/pgtable_e/def.rs (SpecPTE bits)
// Demonstrates: PTE flag checking and manipulation

use vstd::prelude::*;

verus! {

pub struct PTEFlags {
    pub present: bool,
    pub write: bool,
    pub supervisor: bool,
    pub accessed: bool,
    pub dirty: bool,
    pub psize: bool,
    pub global: bool,
    pub encrypted: bool,
    pub nx: bool,
}

impl PTEFlags {
    pub fn new_default() -> (result: Self)
        ensures
            result.present,
            !result.write,
            !result.supervisor,
    {
        PTEFlags {
            present: true,
            write: false,
            supervisor: false,
            accessed: false,
            dirty: false,
            psize: false,
            global: false,
            encrypted: false,
            nx: true,
        }
    }

    pub fn new_writable() -> (result: Self)
        ensures
            result.present,
            result.write,
    {
        PTEFlags {
            present: true,
            write: true,
            supervisor: false,
            accessed: false,
            dirty: false,
            psize: false,
            global: false,
            encrypted: false,
            nx: true,
        }
    }

    pub fn is_present(&self) -> (result: bool)
        ensures
            result <==> self.present,
    {
        self.present
    }

    pub fn is_writable(&self) -> (result: bool)
        ensures
            result <==> self.write,
    {
        self.write
    }

    pub fn is_executable(&self) -> (result: bool)
        ensures
            result <==> !self.nx,
    {
        !self.nx
    }

    pub fn is_encrypted(&self) -> (result: bool)
        ensures
            result <==> self.encrypted,
    {
        self.encrypted
    }

    pub fn is_large_page(&self) -> (result: bool)
        ensures
            result <==> self.psize,
    {
        self.psize
    }

    pub fn is_user_accessible(&self) -> (result: bool)
        ensures
            result <==> !self.supervisor,
    {
        !self.supervisor
    }

    pub fn set_writable(&mut self)
        ensures
            self.write,
            self.present == old(self).present,
    {
        self.write = true;
    }

    pub fn set_encrypted(&mut self)
        ensures
            self.encrypted,
            self.present == old(self).present,
    {
        self.encrypted = true;
    }

    pub fn set_large_page(&mut self)
        ensures
            self.psize,
            self.present == old(self).present,
    {
        self.psize = true;
    }

    pub fn can_read(&self) -> (result: bool)
        ensures
            result <==> self.present,
    {
        self.is_present()
    }

    pub fn can_write(&self) -> (result: bool)
        ensures
            result <==> (self.present && self.write),
    {
        self.is_present() && self.is_writable()
    }

    pub fn can_execute(&self) -> (result: bool)
        ensures
            result <==> (self.present && !self.nx),
    {
        self.is_present() && self.is_executable()
    }
}

fn test_pte_flags() {
    let flags1 = PTEFlags::new_default();
    let flags2 = PTEFlags::new_writable();

    let present = flags1.is_present();
    let writable = flags1.is_writable();
    let executable = flags1.is_executable();

    let encrypted = flags1.is_encrypted();
    let large = flags1.is_large_page();
    let user_access = flags1.is_user_accessible();

    let mut flags3 = PTEFlags::new_default();
    flags3.set_writable();
    let writable_after = flags3.is_writable();

    flags3.set_encrypted();
    let encrypted_after = flags3.is_encrypted();

    flags3.set_large_page();
    let large_after = flags3.is_large_page();

    let can_read = flags3.can_read();
    let can_write = flags3.can_write();
    let can_exec = flags3.can_execute();
}

} // verus!

fn main() {
    test_pte_flags();
}
