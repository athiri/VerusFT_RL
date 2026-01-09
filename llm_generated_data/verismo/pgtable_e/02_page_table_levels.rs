// Variation: Page Table Level Mapping
// From: source/verismo/src/pgtable_e/pte.rs (lvl_to_size)
// Demonstrates: Mapping page table levels to page sizes

use vstd::prelude::*;

verus! {

pub const PAGE_SIZE: usize = 0x1000;           // 4 KB
pub const LARGE_PAGE_SIZE: usize = 0x20_0000;  // 2 MB
pub const HUGE_PAGE_SIZE: usize = 0x4000_0000; // 1 GB

pub const PAGE_TABLE_LEVELS: u8 = 4;

pub open spec fn spec_lvl_to_size(lvl: usize) -> usize {
    if lvl == 0 {
        0x1000
    } else if lvl == 1 {
        0x20_0000
    } else if lvl == 2 {
        0x4000_0000
    } else {
        0
    }
}

pub fn lvl_to_size(lvl: usize) -> (result: usize)
    requires
        lvl <= 2,
    ensures
        result == spec_lvl_to_size(lvl),
{
    if lvl == 0 {
        0x1000
    } else if lvl == 1 {
        0x20_0000
    } else {
        0x4000_0000
    }
}

pub fn is_4kb_page(lvl: usize) -> (result: bool)
    ensures
        result <==> lvl == 0,
{
    lvl == 0
}

pub fn is_2mb_page(lvl: usize) -> (result: bool)
    ensures
        result <==> lvl == 1,
{
    lvl == 1
}

pub fn is_1gb_page(lvl: usize) -> (result: bool)
    ensures
        result <==> lvl == 2,
{
    lvl == 2
}

pub fn get_page_shift(lvl: usize) -> (result: usize)
    requires
        lvl <= 2,
    ensures
        result == if lvl == 0 { 12usize } else if lvl == 1 { 21usize } else { 30usize },
{
    if lvl == 0 {
        12  // 2^12 = 4KB
    } else if lvl == 1 {
        21  // 2^21 = 2MB
    } else {
        30  // 2^30 = 1GB
    }
}

pub fn get_entries_per_table() -> (result: usize)
    ensures
        result == 512,
{
    512
}

pub fn is_valid_entry_index(idx: usize) -> (result: bool)
    ensures
        result <==> idx < 512,
{
    idx < 512
}

pub fn get_level_mask(lvl: usize) -> (result: usize)
    requires
        lvl <= 2,
{
    let size = lvl_to_size(lvl);
    size - 1
}

pub fn is_aligned_to_level(addr: usize, lvl: usize) -> (result: bool)
    requires
        lvl <= 2,
{
    let size = lvl_to_size(lvl);
    addr % size == 0
}

fn test_page_table_levels() {
    let size0 = lvl_to_size(0);
    let size1 = lvl_to_size(1);
    let size2 = lvl_to_size(2);

    let is_4kb = is_4kb_page(0);
    let is_2mb = is_2mb_page(1);
    let is_1gb = is_1gb_page(2);

    let shift0 = get_page_shift(0);
    let shift1 = get_page_shift(1);
    let shift2 = get_page_shift(2);

    let entries = get_entries_per_table();

    let valid_idx = is_valid_entry_index(256);
    let invalid_idx = is_valid_entry_index(512);

    let mask0 = get_level_mask(0);
    let mask1 = get_level_mask(1);

    let aligned = is_aligned_to_level(0x1000, 0);
    let not_aligned = is_aligned_to_level(0x1001, 0);
}

} // verus!

fn main() {
    test_page_table_levels();
}
