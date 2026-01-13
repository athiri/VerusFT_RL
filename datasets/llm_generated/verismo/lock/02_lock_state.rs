// Variation: Lock State Management
// From: source/verismo/src/lock/spin_perm_s.rs (LockPermToRaw concept)
// Demonstrates: Lock state checking and transitions

use vstd::prelude::*;

verus! {

pub struct LockState {
    pub locked: bool,
    pub cpu: usize,
    pub lock_id: usize,
}

impl LockState {
    pub fn new(cpu: usize, lock_id: usize) -> (result: Self)
        ensures
            !result.locked,
            result.cpu == cpu,
            result.lock_id == lock_id,
    {
        LockState { locked: false, cpu, lock_id }
    }

    pub fn is_locked(&self) -> (result: bool)
        ensures
            result <==> self.locked,
    {
        self.locked
    }

    pub fn is_unlocked(&self) -> (result: bool)
        ensures
            result <==> !self.locked,
    {
        !self.locked
    }

    pub fn get_cpu(&self) -> (result: usize)
        ensures
            result == self.cpu,
    {
        self.cpu
    }

    pub fn get_lock_id(&self) -> (result: usize)
        ensures
            result == self.lock_id,
    {
        self.lock_id
    }

    pub fn set_locked(&mut self)
        requires
            !old(self).locked,
        ensures
            self.locked,
            self.cpu == old(self).cpu,
            self.lock_id == old(self).lock_id,
    {
        self.locked = true;
    }

    pub fn set_unlocked(&mut self)
        requires
            old(self).locked,
        ensures
            !self.locked,
            self.cpu == old(self).cpu,
            self.lock_id == old(self).lock_id,
    {
        self.locked = false;
    }

    pub fn is_owned_by(&self, cpu: usize) -> (result: bool)
        ensures
            result <==> (self.cpu == cpu),
    {
        self.cpu == cpu
    }

    pub fn matches_lock(&self, lock_id: usize) -> (result: bool)
        ensures
            result <==> (self.lock_id == lock_id),
    {
        self.lock_id == lock_id
    }

    pub fn can_lock(&self, cpu: usize, lock_id: usize) -> (result: bool)
        ensures
            result <==> (!self.locked && self.cpu == cpu && self.lock_id == lock_id),
    {
        !self.locked && self.cpu == cpu && self.lock_id == lock_id
    }

    pub fn can_unlock(&self, cpu: usize, lock_id: usize) -> (result: bool)
        ensures
            result <==> (self.locked && self.cpu == cpu && self.lock_id == lock_id),
    {
        self.locked && self.cpu == cpu && self.lock_id == lock_id
    }
}

fn test_lock_state() {
    let mut state = LockState::new(0, 100);

    let is_locked = state.is_locked();
    let is_unlocked = state.is_unlocked();
    let cpu = state.get_cpu();
    let lock_id = state.get_lock_id();

    let owned_by_0 = state.is_owned_by(0);
    let owned_by_1 = state.is_owned_by(1);

    let matches = state.matches_lock(100);
    let not_matches = state.matches_lock(200);

    let can_lock_valid = state.can_lock(0, 100);
    let can_lock_wrong_cpu = state.can_lock(1, 100);

    state.set_locked();

    let is_locked_after = state.is_locked();
    let can_unlock_valid = state.can_unlock(0, 100);

    state.set_unlocked();

    let is_unlocked_final = state.is_unlocked();
}

} // verus!

fn main() {
    test_lock_state();
}
