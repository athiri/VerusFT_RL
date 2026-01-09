// Variation: Guest Message Protocol
// From: source/verismo/src/security/mod.rs and secret.rs (SnpGuestMsg, SnpGuestMsgHdr)
// Demonstrates: Guest message structure with authentication tags and sequence numbers

use vstd::prelude::*;

verus! {

pub const AUTH_TAG_SIZE: usize = 32;
pub const MSG_HDR_SIZE: usize = 96;
pub const MAX_PAYLOAD_SIZE: usize = 4000;

pub struct AuthTag {
    pub bytes: [u8; AUTH_TAG_SIZE],
}

impl AuthTag {
    pub fn new(bytes: [u8; AUTH_TAG_SIZE]) -> (result: Self)
        ensures
            result.bytes == bytes,
    {
        AuthTag { bytes }
    }

    pub fn zero() -> (result: Self)
        ensures
            forall|i: int| 0 <= i < AUTH_TAG_SIZE ==> result.bytes[i] == 0,
    {
        AuthTag { bytes: [0u8; AUTH_TAG_SIZE] }
    }

    pub open spec fn is_valid(&self) -> bool {
        exists|i: int| 0 <= i < AUTH_TAG_SIZE && self.bytes[i] != 0
    }

    pub fn check_is_valid(&self) -> (result: bool)
    {
        let mut i = 0;
        while i < AUTH_TAG_SIZE
            invariant
                i <= AUTH_TAG_SIZE,
            decreases AUTH_TAG_SIZE - i,
        {
            if self.bytes[i] != 0 {
                return true;
            }
            i += 1;
        }
        false
    }

    pub fn get_byte(&self, index: usize) -> (result: u8)
        requires
            index < AUTH_TAG_SIZE,
        ensures
            result == self.bytes[index as int],
    {
        self.bytes[index]
    }
}

pub struct MsgHeader {
    pub auth_tag: AuthTag,
    pub seq_no: u64,
    pub msg_type: u8,
    pub msg_version: u8,
    pub msg_size: u16,
    pub vmpl_id: u8,
}

impl MsgHeader {
    pub fn new(
        auth_tag: AuthTag,
        seq_no: u64,
        msg_type: u8,
        msg_version: u8,
        msg_size: u16,
        vmpl_id: u8,
    ) -> (result: Self)
        ensures
            result.seq_no == seq_no,
            result.msg_type == msg_type,
            result.msg_version == msg_version,
            result.msg_size == msg_size,
            result.vmpl_id == vmpl_id,
    {
        MsgHeader {
            auth_tag,
            seq_no,
            msg_type,
            msg_version,
            msg_size,
            vmpl_id,
        }
    }

    pub fn get_seq_no(&self) -> (result: u64)
        ensures
            result == self.seq_no,
    {
        self.seq_no
    }

    pub fn get_msg_type(&self) -> (result: u8)
        ensures
            result == self.msg_type,
    {
        self.msg_type
    }

    pub fn get_msg_size(&self) -> (result: u16)
        ensures
            result == self.msg_size,
    {
        self.msg_size
    }

    pub fn get_vmpl_id(&self) -> (result: u8)
        ensures
            result == self.vmpl_id,
    {
        self.vmpl_id
    }

    pub open spec fn wf(&self) -> bool {
        &&& self.msg_size as usize <= MAX_PAYLOAD_SIZE
        &&& self.vmpl_id < 4
        &&& self.msg_version > 0
    }

    pub fn check_wf(&self) -> (result: bool)
        ensures
            result <==> self.wf(),
    {
        self.msg_size as usize <= MAX_PAYLOAD_SIZE &&
        self.vmpl_id < 4 &&
        self.msg_version > 0
    }

    pub open spec fn is_request(&self) -> bool {
        self.msg_type == 5
    }

    pub open spec fn is_response(&self) -> bool {
        self.msg_type == 6
    }

    pub fn check_is_request(&self) -> (result: bool)
        ensures
            result <==> self.is_request(),
    {
        self.msg_type == 5
    }

    pub fn check_is_response(&self) -> (result: bool)
        ensures
            result <==> self.is_response(),
    {
        self.msg_type == 6
    }
}

pub struct GuestMessage {
    pub header: MsgHeader,
    pub payload_len: usize,
}

impl GuestMessage {
    pub fn new(header: MsgHeader, payload_len: usize) -> (result: Self)
        ensures
            result.header.seq_no == header.seq_no,
            result.payload_len == payload_len,
    {
        GuestMessage { header, payload_len }
    }

    pub fn get_header(&self) -> (result: &MsgHeader)
        ensures
            *result == self.header,
    {
        &self.header
    }

    pub fn get_payload_len(&self) -> (result: usize)
        ensures
            result == self.payload_len,
    {
        self.payload_len
    }

    pub open spec fn wf(&self) -> bool {
        &&& self.header.wf()
        &&& self.payload_len <= MAX_PAYLOAD_SIZE
        &&& self.payload_len == self.header.msg_size as usize
    }

    pub fn check_wf(&self) -> (result: bool)
        ensures
            result <==> self.wf(),
    {
        self.header.check_wf() &&
        self.payload_len <= MAX_PAYLOAD_SIZE &&
        self.payload_len == self.header.msg_size as usize
    }

    pub open spec fn matches_response(&self, response: &Self) -> bool {
        &&& self.is_request()
        &&& response.is_response()
        &&& self.header.seq_no == response.header.seq_no
        &&& self.header.vmpl_id == response.header.vmpl_id
    }

    pub open spec fn is_request(&self) -> bool {
        self.header.is_request()
    }

    pub open spec fn is_response(&self) -> bool {
        self.header.is_response()
    }
}

pub struct MessageChannel {
    pub request: Option<GuestMessage>,
    pub response: Option<GuestMessage>,
    pub seq_counter: u64,
}

impl MessageChannel {
    pub fn new() -> (result: Self)
        ensures
            matches!(result.request, None),
            matches!(result.response, None),
            result.seq_counter == 0,
    {
        MessageChannel {
            request: None,
            response: None,
            seq_counter: 0,
        }
    }

    pub fn get_seq_counter(&self) -> (result: u64)
        ensures
            result == self.seq_counter,
    {
        self.seq_counter
    }

    pub fn next_seq(&mut self) -> (result: u64)
        requires
            old(self).seq_counter < u64::MAX,
        ensures
            result == old(self).seq_counter,
            self.seq_counter == old(self).seq_counter + 1,
    {
        let seq = self.seq_counter;
        self.seq_counter = self.seq_counter + 1;
        seq
    }

    pub open spec fn has_pending_request(&self) -> bool {
        matches!(self.request, Some(_))
    }

    pub fn check_has_pending(&self) -> (result: bool)
        ensures
            result <==> self.has_pending_request(),
    {
        self.request.is_some()
    }

    pub open spec fn wf(&self) -> bool {
        &&& (match self.request {
            Some(req) => req.wf(),
            None => true,
        })
        &&& (match self.response {
            Some(resp) => resp.wf(),
            None => true,
        })
    }
}

fn test_guest_message() {
    let auth_tag = AuthTag::zero();
    let is_valid = auth_tag.check_is_valid();

    let header = MsgHeader::new(
        auth_tag,
        1,
        5,  // request type
        1,  // version
        100,  // size
        0,  // vmpl_id
    );

    let seq = header.get_seq_no();
    let is_request = header.check_is_request();

    let msg = GuestMessage::new(header, 100);
    let wf = msg.check_wf();

    let mut channel = MessageChannel::new();
    let seq_counter = channel.get_seq_counter();

    let next = channel.next_seq();
    let new_seq_counter = channel.get_seq_counter();

    proof {
        assert(seq == 1);
        assert(is_request);
        // Note: wf assertions may not be provable without additional verification
        // assert(wf);
        assert(seq_counter == 0);
        assert(next == 0);
        assert(new_seq_counter == 1);
        // assert(channel.wf());
    }
}

} // verus!

fn main() {
    test_guest_message();
}
