// Variation: PTE Attributes
// From: source/verismo/src/pgtable_e/pte.rs (to_attr, has_next)
// Demonstrates: Page table entry attribute checking

use vstd::prelude::*;

verus! {

pub struct PTAttributes {
    pub encrypted: bool,
    pub executable: bool,
    pub writable: bool,
}

impl PTAttributes {
    pub fn new_default() -> (result: Self)
        ensures
            !result.encrypted,
            !result.executable,
            !result.writable,
    {
        PTAttributes {
            encrypted: false,
            executable: false,
            writable: false,
        }
    }

    pub fn new_rwx() -> (result: Self)
        ensures
            result.writable,
            result.executable,
    {
        PTAttributes {
            encrypted: false,
            executable: true,
            writable: true,
        }
    }

    pub fn is_encrypted(&self) -> (result: bool)
        ensures
            result <==> self.encrypted,
    {
        self.encrypted
    }

    pub fn is_executable(&self) -> (result: bool)
        ensures
            result <==> self.executable,
    {
        self.executable
    }

    pub fn is_writable(&self) -> (result: bool)
        ensures
            result <==> self.writable,
    {
        self.writable
    }

    pub fn is_readonly(&self) -> (result: bool)
        ensures
            result <==> !self.writable,
    {
        !self.writable
    }

    pub fn matches(&self, other: &PTAttributes) -> (result: bool)
        ensures
            result <==> (self.encrypted == other.encrypted
                      && self.executable == other.executable
                      && self.writable == other.writable),
    {
        self.encrypted == other.encrypted
            && self.executable == other.executable
            && self.writable == other.writable
    }

    pub fn set_encrypted(&mut self)
        ensures
            self.encrypted,
            self.executable == old(self).executable,
            self.writable == old(self).writable,
    {
        self.encrypted = true;
    }

    pub fn set_writable(&mut self)
        ensures
            self.writable,
            self.encrypted == old(self).encrypted,
            self.executable == old(self).executable,
    {
        self.writable = true;
    }

    pub fn set_executable(&mut self)
        ensures
            self.executable,
            self.encrypted == old(self).encrypted,
            self.writable == old(self).writable,
    {
        self.executable = true;
    }
}

pub struct PTEInfo {
    pub present: bool,
    pub psize: bool,
}

impl PTEInfo {
    pub fn new(present: bool, psize: bool) -> (result: Self)
        ensures
            result.present == present,
            result.psize == psize,
    {
        PTEInfo { present, psize }
    }

    pub fn has_next(&self) -> (result: bool)
        ensures
            result <==> (self.psize == false && self.present == true),
    {
        self.psize == false && self.present == true
    }

    pub fn is_leaf(&self) -> (result: bool)
        ensures
            result <==> !(self.psize == false && self.present == true),
    {
        !self.has_next()
    }

    pub fn is_large_page(&self) -> (result: bool)
        ensures
            result <==> (self.present && self.psize),
    {
        self.present && self.psize
    }
}

fn test_pte_attributes() {
    let attrs1 = PTAttributes::new_default();
    let attrs2 = PTAttributes::new_rwx();

    let encrypted = attrs1.is_encrypted();
    let executable = attrs1.is_executable();
    let writable = attrs1.is_writable();
    let readonly = attrs1.is_readonly();

    let match12 = attrs1.matches(&attrs2);

    let mut attrs3 = PTAttributes::new_default();
    attrs3.set_encrypted();
    attrs3.set_writable();
    attrs3.set_executable();

    let info1 = PTEInfo::new(true, false);
    let info2 = PTEInfo::new(true, true);

    let has_next1 = info1.has_next();
    let has_next2 = info2.has_next();

    let is_leaf1 = info1.is_leaf();
    let is_large = info2.is_large_page();
}

} // verus!

fn main() {
    test_pte_attributes();
}
