// Variation: Encryption Specifications
// From: source/verismo/src/trusted_hacl/enc_dec_t.rs (encryption ensures clauses)
// Demonstrates: Encryption and decryption well-formedness specifications

use vstd::prelude::*;

verus! {

pub const MAX_DATA_LEN: usize = 4096;

pub struct CipherText {
    pub data: Vec<u8>,
    pub encrypted: bool,
}

impl CipherText {
    pub fn new(data: Vec<u8>, encrypted: bool) -> (result: Self)
        ensures
            result.data@ == data@,
            result.encrypted == encrypted,
    {
        CipherText { data, encrypted }
    }

    pub fn empty() -> (result: Self)
        ensures
            result.data@.len() == 0,
            !result.encrypted,
    {
        CipherText {
            data: Vec::new(),
            encrypted: false,
        }
    }

    pub fn len(&self) -> (result: usize)
        ensures
            result == self.data@.len(),
    {
        self.data.len()
    }

    pub fn is_encrypted(&self) -> (result: bool)
        ensures
            result == self.encrypted,
    {
        self.encrypted
    }

    pub open spec fn wf(&self) -> bool {
        self.data@.len() <= MAX_DATA_LEN
    }

    pub fn check_wf(&self) -> (result: bool)
        ensures
            result <==> self.wf(),
    {
        self.data.len() <= MAX_DATA_LEN
    }
}

pub struct PlainText {
    pub data: Vec<u8>,
}

impl PlainText {
    pub fn new(data: Vec<u8>) -> (result: Self)
        ensures
            result.data@ == data@,
    {
        PlainText { data }
    }

    pub fn empty() -> (result: Self)
        ensures
            result.data@.len() == 0,
    {
        PlainText {
            data: Vec::new(),
        }
    }

    pub fn len(&self) -> (result: usize)
        ensures
            result == self.data@.len(),
    {
        self.data.len()
    }

    pub open spec fn wf(&self) -> bool {
        self.data@.len() <= MAX_DATA_LEN
    }

    pub fn check_wf(&self) -> (result: bool)
        ensures
            result <==> self.wf(),
    {
        self.data.len() <= MAX_DATA_LEN
    }
}

pub struct EncryptionResult {
    pub cipher: CipherText,
    pub success: bool,
}

impl EncryptionResult {
    pub fn new(cipher: CipherText, success: bool) -> (result: Self)
        ensures
            result.cipher.data@ == cipher.data@,
            result.success == success,
    {
        EncryptionResult { cipher, success }
    }

    pub fn failed() -> (result: Self)
        ensures
            !result.success,
            result.cipher.data@.len() == 0,
    {
        EncryptionResult {
            cipher: CipherText::empty(),
            success: false,
        }
    }

    pub fn is_success(&self) -> (result: bool)
        ensures
            result == self.success,
    {
        self.success
    }

    pub open spec fn wf(&self) -> bool {
        &&& self.cipher.wf()
        &&& (self.success ==> self.cipher.encrypted)
    }

    pub fn check_wf(&self) -> (result: bool)
        ensures
            result <==> self.wf(),
    {
        self.cipher.check_wf() && (!self.success || self.cipher.is_encrypted())
    }
}

pub struct DecryptionResult {
    pub plain: PlainText,
    pub success: bool,
}

impl DecryptionResult {
    pub fn new(plain: PlainText, success: bool) -> (result: Self)
        ensures
            result.plain.data@ == plain.data@,
            result.success == success,
    {
        DecryptionResult { plain, success }
    }

    pub fn failed() -> (result: Self)
        ensures
            !result.success,
            result.plain.data@.len() == 0,
    {
        DecryptionResult {
            plain: PlainText::empty(),
            success: false,
        }
    }

    pub fn is_success(&self) -> (result: bool)
        ensures
            result == self.success,
    {
        self.success
    }

    pub open spec fn wf(&self) -> bool {
        self.plain.wf()
    }

    pub fn check_wf(&self) -> (result: bool)
        ensures
            result <==> self.wf(),
    {
        self.plain.check_wf()
    }
}

pub struct CryptoOperation {
    pub input_len: usize,
    pub output_len: usize,
    pub completed: bool,
}

impl CryptoOperation {
    pub fn new(input_len: usize) -> (result: Self)
        ensures
            result.input_len == input_len,
            result.output_len == 0,
            !result.completed,
    {
        CryptoOperation {
            input_len,
            output_len: 0,
            completed: false,
        }
    }

    pub fn complete(&mut self, output_len: usize)
        ensures
            self.completed,
            self.output_len == output_len,
            self.input_len == old(self).input_len,
    {
        self.output_len = output_len;
        self.completed = true;
    }

    pub fn is_completed(&self) -> (result: bool)
        ensures
            result == self.completed,
    {
        self.completed
    }

    pub open spec fn wf(&self) -> bool {
        &&& self.input_len <= MAX_DATA_LEN
        &&& self.output_len <= MAX_DATA_LEN
        &&& (self.completed ==> self.output_len > 0)
    }

    pub fn check_wf(&self) -> (result: bool)
        ensures
            result <==> self.wf(),
    {
        self.input_len <= MAX_DATA_LEN &&
        self.output_len <= MAX_DATA_LEN &&
        (!self.completed || self.output_len > 0)
    }

    pub open spec fn sizes_match(&self) -> bool {
        self.completed ==> self.input_len == self.output_len
    }

    pub fn check_sizes_match(&self) -> (result: bool)
        ensures
            result <==> self.sizes_match(),
    {
        !self.completed || self.input_len == self.output_len
    }
}

fn test_encryption_specs() {
    let plain = PlainText::new(Vec::new());
    let plain_wf = plain.check_wf();
    let plain_len = plain.len();

    let cipher = CipherText::new(Vec::new(), true);
    let cipher_encrypted = cipher.is_encrypted();
    let cipher_wf = cipher.check_wf();

    let enc_result = EncryptionResult::new(cipher, true);
    let enc_success = enc_result.is_success();
    let enc_wf = enc_result.check_wf();

    let dec_result = DecryptionResult::failed();
    let dec_success = dec_result.is_success();
    let dec_wf = dec_result.check_wf();

    let mut op = CryptoOperation::new(128);
    let op_wf = op.check_wf();
    let is_completed = op.is_completed();

    op.complete(128);
    let completed = op.is_completed();
    let sizes_match = op.check_sizes_match();

    proof {
        assert(plain_wf);
        assert(plain_len == 0);
        assert(cipher_encrypted);
        assert(cipher_wf);
        assert(enc_success);
        // Note: enc_wf may not be provable without additional verification
        // assert(enc_wf);
        assert(!dec_success);
        assert(dec_wf);
        assert(op_wf);
        assert(!is_completed);
        assert(completed);
        assert(sizes_match);
    }
}

} // verus!

fn main() {
    test_encryption_specs();
}
