// Variation: Virtual to Physical Address Conversion
// From: source/verismo/src/pgtable_e/pte.rs (va_to_pa, pa_to_va)
// Demonstrates: Address conversion helpers (identity mapping assumed)

use vstd::prelude::*;

verus! {

pub const PAGE_SIZE: usize = 0x1000;

pub open spec fn spec_addr_to_page(addr: usize) -> usize {
    addr / PAGE_SIZE
}

pub open spec fn spec_page_to_addr(page: usize) -> usize {
    (page * PAGE_SIZE) as usize
}

pub fn addr_to_page(addr: usize) -> (result: usize)
    ensures
        result == spec_addr_to_page(addr),
{
    addr / PAGE_SIZE
}

pub fn page_to_addr(page: usize) -> (result: usize)
    requires
        page <= usize::MAX / PAGE_SIZE,
    ensures
        result == spec_page_to_addr(page),
{
    page * PAGE_SIZE
}

pub fn get_page_offset(addr: usize) -> (result: usize)
    ensures
        result == addr % PAGE_SIZE,
        result < PAGE_SIZE,
{
    addr % PAGE_SIZE
}

pub fn get_page_base(addr: usize) -> (result: usize)
    ensures
        result == addr - (addr % PAGE_SIZE),
        result % PAGE_SIZE == 0,
{
    addr - get_page_offset(addr)
}

pub fn is_page_aligned(addr: usize) -> (result: bool)
    ensures
        result <==> addr % PAGE_SIZE == 0,
{
    addr % PAGE_SIZE == 0
}

pub fn align_down_to_page(addr: usize) -> (result: usize)
    ensures
        result <= addr,
        result % PAGE_SIZE == 0,
{
    get_page_base(addr)
}

pub fn align_up_to_page(addr: usize) -> (result: usize)
    requires
        addr <= usize::MAX - PAGE_SIZE,
    ensures
        result >= addr,
        result % PAGE_SIZE == 0,
{
    let offset = get_page_offset(addr);
    if offset == 0 {
        addr
    } else {
        addr + (PAGE_SIZE - offset)
    }
}

pub fn pages_between(start: usize, end: usize) -> (result: usize)
    requires
        start <= end,
    ensures
        result as int == (end - start) as int / PAGE_SIZE as int,
{
    (end - start) / PAGE_SIZE
}

pub fn offset_in_same_page(addr1: usize, addr2: usize) -> (result: bool)
    ensures
        result <==> (addr1 / PAGE_SIZE == addr2 / PAGE_SIZE),
{
    addr_to_page(addr1) == addr_to_page(addr2)
}

fn test_address_conversion() {
    let addr = 0x12345;
    let page_num = addr_to_page(addr);
    let reconstructed = page_to_addr(page_num);

    let offset = get_page_offset(addr);
    let base = get_page_base(addr);

    let aligned = is_page_aligned(0x1000);
    let not_aligned = is_page_aligned(0x1001);

    let down = align_down_to_page(0x1234);
    let up = align_up_to_page(0x1234);

    let pages = pages_between(0x1000, 0x3000);

    let same_page = offset_in_same_page(0x1000, 0x1500);
    let diff_page = offset_in_same_page(0x1000, 0x2000);
}

} // verus!

fn main() {
    test_address_conversion();
}
