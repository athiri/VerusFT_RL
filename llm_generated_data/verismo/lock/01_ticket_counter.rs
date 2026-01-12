// Variation: Ticket Counter Operations
// From: source/verismo/src/lock/spin_t.rs (ticket lock concept)
// Demonstrates: Ticket-based lock counter logic

use vstd::prelude::*;

verus! {

pub struct TicketCounter {
    pub current: u64,
    pub holder: u64,
}

impl TicketCounter {
    pub fn new() -> (result: Self)
        ensures
            result.current == 1,
            result.holder == 1,
    {
        TicketCounter { current: 1, holder: 1 }
    }

    pub fn get_current(&self) -> (result: u64)
        ensures
            result == self.current,
    {
        self.current
    }

    pub fn get_holder(&self) -> (result: u64)
        ensures
            result == self.holder,
    {
        self.holder
    }

    pub fn increment_current(&mut self) -> (result: u64)
        requires
            old(self).current < u64::MAX,
        ensures
            self.current == old(self).current + 1,
            self.holder == old(self).holder,
            result == old(self).current,
    {
        let ticket = self.current;
        self.current = self.current + 1;
        ticket
    }

    pub fn increment_holder(&mut self)
        requires
            old(self).holder < u64::MAX,
        ensures
            self.holder == old(self).holder + 1,
            self.current == old(self).current,
    {
        self.holder = self.holder + 1;
    }

    pub fn is_my_turn(&self, ticket: u64) -> (result: bool)
        ensures
            result <==> self.holder == ticket,
    {
        self.holder == ticket
    }

    pub fn tickets_ahead(&self, ticket: u64) -> (result: u64)
        requires
            ticket <= self.current,
            ticket <= self.holder,
        ensures
            result == self.holder - ticket,
    {
        self.holder - ticket
    }

    pub fn can_acquire(&self, ticket: u64) -> (result: bool)
        ensures
            result <==> (ticket <= self.current && self.holder == ticket),
    {
        ticket <= self.current && self.holder == ticket
    }
}

fn test_ticket_counter() {
    let mut counter = TicketCounter::new();

    let ticket1 = counter.increment_current();
    let ticket2 = counter.increment_current();
    let ticket3 = counter.increment_current();

    let is_turn_1 = counter.is_my_turn(ticket1);
    let is_turn_2 = counter.is_my_turn(ticket2);

    let ahead1 = counter.tickets_ahead(ticket1);

    counter.increment_holder();

    let is_turn_1_after = counter.is_my_turn(ticket1);
    let is_turn_2_after = counter.is_my_turn(ticket2);

    let ahead2 = counter.tickets_ahead(ticket2);

    let can_acq_1 = counter.can_acquire(ticket1);
    let can_acq_2 = counter.can_acquire(ticket2);
}

} // verus!

fn main() {
    test_ticket_counter();
}
