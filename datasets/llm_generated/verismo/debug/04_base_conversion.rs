// Variation: Base Conversion Helpers
// From: source/verismo/src/debug/ghcb_print.rs (int2bytes concept)
// Demonstrates: Integer to string conversion in different bases

use vstd::prelude::*;

verus! {

pub open spec fn is_valid_base(base: u64) -> bool {
    1 < base && base <= 16
}

pub fn check_valid_base(base: u64) -> (result: bool)
    ensures
        result <==> is_valid_base(base),
{
    1 < base && base <= 16
}

pub fn digit_in_base(n: u64, base: u64) -> (result: u8)
    requires
        is_valid_base(base),
        n < base,
    ensures
        result < 16,
{
    (n % base) as u8
}

pub fn divide_by_base(n: u64, base: u64) -> (result: u64)
    requires
        is_valid_base(base),
    ensures
        result == n / base,
        result <= n,
{
    n / base
}

pub fn count_digits_needed(n: u64, base: u64) -> (result: usize)
    requires
        is_valid_base(base),
    ensures
        result <= 64,
{
    if n == 0 {
        1
    } else {
        let mut count = 0;
        let mut val = n;
        while val > 0 && count < 64
            invariant
                0 <= count <= 64,
                val <= n,
                is_valid_base(base),
            decreases 64 - count
        {
            val = val / base;
            count = count + 1;
        }
        if count == 0 {
            1
        } else {
            count
        }
    }
}

pub fn get_base_prefix(base: u64) -> (result: u8)
    requires
        is_valid_base(base),
    ensures
        result == 'x' as u8 || result == 'h' as u8 || result == 'b' as u8 || result == '_' as u8,
{
    if base == 16 {
        'x' as u8
    } else if base == 8 {
        'h' as u8  // octal
    } else if base == 2 {
        'b' as u8  // binary
    } else {
        '_' as u8  // other
    }
}

pub fn is_power_of_two_base(base: u64) -> (result: bool)
    ensures
        result <==> (base == 2 || base == 4 || base == 8 || base == 16),
{
    base == 2 || base == 4 || base == 8 || base == 16
}

fn test_base_conversion() {
    let valid2 = check_valid_base(2);
    let valid10 = check_valid_base(10);
    let valid16 = check_valid_base(16);
    let valid20 = check_valid_base(20);

    let digit = digit_in_base(15, 16);
    let quotient = divide_by_base(100, 10);

    let digits1 = count_digits_needed(255, 10);
    let digits2 = count_digits_needed(255, 16);

    let prefix16 = get_base_prefix(16);
    let prefix8 = get_base_prefix(8);
    let prefix2 = get_base_prefix(2);

    let pow_of_two_16 = is_power_of_two_base(16);
    let pow_of_two_10 = is_power_of_two_base(10);
}

} // verus!

fn main() {
    test_base_conversion();
}
