// Variation: Retry Logic
// From: source/verismo/src/mshyper/hypercall.rs (hv_call_with_retry)
// Demonstrates: Retry counter and logic for hypercalls

use vstd::prelude::*;

verus! {

pub const HV_MAX_RETRY: usize = 10;
pub const HV_STATUS_TIMEOUT: u64 = 0x78;
pub const HV_STATUS_SUCCESS: u64 = 0x0;

pub struct RetryState {
    pub attempts: usize,
    pub max_retries: usize,
    pub last_status: u64,
}

impl RetryState {
    pub fn new(max_retries: usize) -> (result: Self)
        ensures
            result.attempts == 0,
            result.max_retries == max_retries,
            result.last_status == 0,
    {
        RetryState {
            attempts: 0,
            max_retries,
            last_status: 0,
        }
    }

    pub fn new_default() -> (result: Self)
        ensures
            result.attempts == 0,
            result.max_retries == HV_MAX_RETRY,
    {
        Self::new(HV_MAX_RETRY)
    }

    pub fn should_retry(&self) -> (result: bool)
        ensures
            result <==> (self.attempts < self.max_retries),
    {
        self.attempts < self.max_retries
    }

    pub fn can_continue(&self, status: u64) -> (result: bool)
        ensures
            result <==> (self.attempts < self.max_retries && status == HV_STATUS_TIMEOUT),
    {
        self.should_retry() && status == HV_STATUS_TIMEOUT
    }

    pub fn increment(&mut self, status: u64)
        requires
            old(self).attempts < usize::MAX,
        ensures
            self.attempts == old(self).attempts + 1,
            self.last_status == status,
            self.max_retries == old(self).max_retries,
    {
        self.attempts = self.attempts + 1;
        self.last_status = status;
    }

    pub fn get_attempts(&self) -> (result: usize)
        ensures
            result == self.attempts,
    {
        self.attempts
    }

    pub fn get_last_status(&self) -> (result: u64)
        ensures
            result == self.last_status,
    {
        self.last_status
    }

    pub fn is_exhausted(&self) -> (result: bool)
        ensures
            result <==> self.attempts >= self.max_retries,
    {
        self.attempts >= self.max_retries
    }

    pub fn is_success(&self) -> (result: bool)
        ensures
            result <==> self.last_status == HV_STATUS_SUCCESS,
    {
        self.last_status == HV_STATUS_SUCCESS
    }

    pub fn reset(&mut self)
        ensures
            self.attempts == 0,
            self.last_status == 0,
            self.max_retries == old(self).max_retries,
    {
        self.attempts = 0;
        self.last_status = 0;
    }
}

fn test_retry_logic() {
    let mut state = RetryState::new_default();

    let should_retry1 = state.should_retry();
    let can_continue1 = state.can_continue(HV_STATUS_TIMEOUT);

    state.increment(HV_STATUS_TIMEOUT);

    let attempts1 = state.get_attempts();
    let status1 = state.get_last_status();

    state.increment(HV_STATUS_SUCCESS);

    let is_success = state.is_success();
    let exhausted = state.is_exhausted();

    state.reset();

    let attempts_after_reset = state.get_attempts();
}

} // verus!

fn main() {
    test_retry_logic();
}
