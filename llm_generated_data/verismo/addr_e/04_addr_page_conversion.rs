// Variation: Address and Page Number Conversion
// From: source/verismo/src/addr_e/addr_interface.rs
// Demonstrates: Address to page conversion, page to address conversion

use vstd::prelude::*;

verus! {

pub const PAGE_SIZE: usize = 4096;
pub const MAX_PAGES: usize = 0x100000; // 1M pages

pub fn addr_to_page(addr: usize) -> (result: usize)
    ensures
        result == addr / PAGE_SIZE,
{
    addr / PAGE_SIZE
}

pub fn page_to_addr(page: usize) -> (result: usize)
    requires
        page <= usize::MAX / PAGE_SIZE,
    ensures
        result == page * PAGE_SIZE,
{
    page * PAGE_SIZE
}

pub fn is_page_boundary(addr: usize) -> (result: bool)
    ensures
        result <==> (addr % PAGE_SIZE == 0),
{
    addr % PAGE_SIZE == 0
}

pub fn page_offset(addr: usize) -> (result: usize)
    ensures
        result == addr % PAGE_SIZE,
        result < PAGE_SIZE,
{
    addr % PAGE_SIZE
}

pub fn page_base(addr: usize) -> (result: usize)
    ensures
        result == (addr / PAGE_SIZE) * PAGE_SIZE,
        result <= addr,
        result % PAGE_SIZE == 0,
{
    (addr / PAGE_SIZE) * PAGE_SIZE
}

pub fn addr_in_page(addr: usize, page: usize) -> (result: bool)
    requires
        page <= usize::MAX / PAGE_SIZE,
        page < usize::MAX / PAGE_SIZE,
    ensures
        result <==> (page * PAGE_SIZE <= addr && addr < (page + 1) * PAGE_SIZE),
{
    let page_start = page_to_addr(page);
    if page_start <= usize::MAX - PAGE_SIZE {
        page_start <= addr && addr < page_start + PAGE_SIZE
    } else {
        false
    }
}

pub fn pages_between(start_addr: usize, end_addr: usize) -> (result: usize)
    requires
        start_addr <= end_addr,
    ensures
        result * PAGE_SIZE <= end_addr - start_addr,
{
    (end_addr - start_addr) / PAGE_SIZE
}

fn test_addr_page_conversion() {
    let addr1 = 0x1000usize;
    let page1 = addr_to_page(addr1);
    assert(page1 == 1);

    let addr2 = page_to_addr(10);
    assert(addr2 == 0xa000);

    let boundary1 = is_page_boundary(0x2000);
    assert(boundary1);

    let boundary2 = is_page_boundary(0x2001);
    assert(!boundary2);

    let offset1 = page_offset(0x1234);
    assert(offset1 == 0x234);
    assert(offset1 < PAGE_SIZE);

    let base1 = page_base(0x1234);
    assert(base1 == 0x1000);
    assert(base1 % PAGE_SIZE == 0);

    let in_page = addr_in_page(0x1234, 1);
    assert(in_page);

    let not_in_page = addr_in_page(0x2234, 1);
    assert(!not_in_page);

    let pages = pages_between(0x1000, 0x5000);
    assert(pages * PAGE_SIZE <= 0x4000);
}

} // verus!

fn main() {
    test_addr_page_conversion();
}
