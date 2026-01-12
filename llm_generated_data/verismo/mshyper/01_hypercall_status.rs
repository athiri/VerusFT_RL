// Variation: Hypercall Status Checking
// From: source/verismo/src/mshyper/mod.rs, hypercall.rs
// Demonstrates: Hypercall status codes and checking

use vstd::prelude::*;

verus! {

pub type HvCallStatus = u64;

pub const HV_STATUS_SUCCESS: u64 = 0x0;
pub const HV_STATUS_TIMEOUT: u64 = 0x78;
pub const HV_STATUS_INVALID_PARAMETER: u64 = 0x5;
pub const HV_STATUS_ACCESS_DENIED: u64 = 0x6;

pub fn is_status_success(status: HvCallStatus) -> (result: bool)
    ensures
        result <==> status == HV_STATUS_SUCCESS,
{
    status == HV_STATUS_SUCCESS
}

pub fn is_status_timeout(status: HvCallStatus) -> (result: bool)
    ensures
        result <==> status == HV_STATUS_TIMEOUT,
{
    status == HV_STATUS_TIMEOUT
}

pub fn is_status_error(status: HvCallStatus) -> (result: bool)
    ensures
        result <==> status != HV_STATUS_SUCCESS,
{
    status != HV_STATUS_SUCCESS
}

pub fn should_retry(status: HvCallStatus) -> (result: bool)
    ensures
        result <==> status == HV_STATUS_TIMEOUT,
{
    is_status_timeout(status)
}

pub fn is_parameter_error(status: HvCallStatus) -> (result: bool)
    ensures
        result <==> status == HV_STATUS_INVALID_PARAMETER,
{
    status == HV_STATUS_INVALID_PARAMETER
}

pub fn is_access_denied(status: HvCallStatus) -> (result: bool)
    ensures
        result <==> status == HV_STATUS_ACCESS_DENIED,
{
    status == HV_STATUS_ACCESS_DENIED
}

pub fn get_status_code(status: HvCallStatus) -> (result: u32)
    ensures
        result == status as u32,
{
    status as u32
}

pub fn check_status_range(status: HvCallStatus, max: u64) -> (result: bool)
    ensures
        result <==> status <= max,
{
    status <= max
}

fn test_hypercall_status() {
    let success = is_status_success(HV_STATUS_SUCCESS);
    let not_success = is_status_success(HV_STATUS_TIMEOUT);

    let timeout = is_status_timeout(HV_STATUS_TIMEOUT);
    let not_timeout = is_status_timeout(HV_STATUS_SUCCESS);

    let error1 = is_status_error(HV_STATUS_TIMEOUT);
    let error2 = is_status_error(HV_STATUS_SUCCESS);

    let retry1 = should_retry(HV_STATUS_TIMEOUT);
    let retry2 = should_retry(HV_STATUS_SUCCESS);

    let param_err = is_parameter_error(HV_STATUS_INVALID_PARAMETER);
    let access_denied = is_access_denied(HV_STATUS_ACCESS_DENIED);

    let code = get_status_code(HV_STATUS_TIMEOUT);

    let in_range = check_status_range(HV_STATUS_TIMEOUT, 0x100);
}

} // verus!

fn main() {
    test_hypercall_status();
}
