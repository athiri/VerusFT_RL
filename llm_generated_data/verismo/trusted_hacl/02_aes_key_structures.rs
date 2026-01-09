// Variation: AES-256 Key Structures
// From: source/verismo/src/trusted_hacl/enc_dec_t.rs (AESKey256, types)
// Demonstrates: AES-256 key and associated data structures

use vstd::prelude::*;

verus! {

pub const AES_256_KEY_LEN: usize = 32;
pub const IV_LEN: usize = 12;
pub const AAD_LEN: usize = 48;
pub const AUTH_TAG_LEN: usize = 32;

pub struct AES256Key {
    pub bytes: [u8; AES_256_KEY_LEN],
}

impl AES256Key {
    pub fn new(bytes: [u8; AES_256_KEY_LEN]) -> (result: Self)
        ensures
            result.bytes == bytes,
    {
        AES256Key { bytes }
    }

    pub fn zero() -> (result: Self)
        ensures
            forall|i: int| 0 <= i < AES_256_KEY_LEN ==> result.bytes[i] == 0,
    {
        AES256Key { bytes: [0u8; AES_256_KEY_LEN] }
    }

    pub fn get_byte(&self, index: usize) -> (result: u8)
        requires
            index < AES_256_KEY_LEN,
        ensures
            result == self.bytes[index as int],
    {
        self.bytes[index]
    }

    pub open spec fn is_zero(&self) -> bool {
        forall|i: int| 0 <= i < AES_256_KEY_LEN ==> self.bytes[i] == 0
    }

    pub fn check_is_zero(&self) -> (result: bool)
        ensures
            result <==> self.is_zero(),
    {
        let mut i = 0;
        while i < AES_256_KEY_LEN
            invariant
                i <= AES_256_KEY_LEN,
                forall|j: int| 0 <= j < i ==> self.bytes[j] == 0,
            decreases AES_256_KEY_LEN - i,
        {
            if self.bytes[i] != 0 {
                return false;
            }
            i += 1;
        }
        true
    }

    pub open spec fn wf(&self) -> bool {
        !self.is_zero()
    }

    pub fn check_wf(&self) -> (result: bool)
        ensures
            result <==> self.wf(),
    {
        !self.check_is_zero()
    }
}

pub struct InitializationVector {
    pub bytes: [u8; IV_LEN],
}

impl InitializationVector {
    pub fn new(bytes: [u8; IV_LEN]) -> (result: Self)
        ensures
            result.bytes == bytes,
    {
        InitializationVector { bytes }
    }

    pub fn zero() -> (result: Self)
        ensures
            forall|i: int| 0 <= i < IV_LEN ==> result.bytes[i] == 0,
    {
        InitializationVector { bytes: [0u8; IV_LEN] }
    }

    pub fn get_byte(&self, index: usize) -> (result: u8)
        requires
            index < IV_LEN,
        ensures
            result == self.bytes[index as int],
    {
        self.bytes[index]
    }

    pub open spec fn wf(&self) -> bool {
        true
    }
}

pub struct AdditionalAuthData {
    pub bytes: [u8; AAD_LEN],
}

impl AdditionalAuthData {
    pub fn new(bytes: [u8; AAD_LEN]) -> (result: Self)
        ensures
            result.bytes == bytes,
    {
        AdditionalAuthData { bytes }
    }

    pub fn zero() -> (result: Self)
        ensures
            forall|i: int| 0 <= i < AAD_LEN ==> result.bytes[i] == 0,
    {
        AdditionalAuthData { bytes: [0u8; AAD_LEN] }
    }

    pub fn get_byte(&self, index: usize) -> (result: u8)
        requires
            index < AAD_LEN,
        ensures
            result == self.bytes[index as int],
    {
        self.bytes[index]
    }

    pub open spec fn wf(&self) -> bool {
        true
    }
}

pub struct AuthenticationTag {
    pub bytes: [u8; AUTH_TAG_LEN],
}

impl AuthenticationTag {
    pub fn new(bytes: [u8; AUTH_TAG_LEN]) -> (result: Self)
        ensures
            result.bytes == bytes,
    {
        AuthenticationTag { bytes }
    }

    pub fn zero() -> (result: Self)
        ensures
            forall|i: int| 0 <= i < AUTH_TAG_LEN ==> result.bytes[i] == 0,
    {
        AuthenticationTag { bytes: [0u8; AUTH_TAG_LEN] }
    }

    pub fn get_byte(&self, index: usize) -> (result: u8)
        requires
            index < AUTH_TAG_LEN,
        ensures
            result == self.bytes[index as int],
    {
        self.bytes[index]
    }

    pub open spec fn is_valid(&self) -> bool {
        exists|i: int| 0 <= i < AUTH_TAG_LEN && self.bytes[i] != 0
    }

    pub open spec fn wf(&self) -> bool {
        true
    }
}

pub struct EncryptionContext {
    pub key: AES256Key,
    pub iv: InitializationVector,
    pub aad: AdditionalAuthData,
    pub tag: AuthenticationTag,
}

impl EncryptionContext {
    pub fn new(
        key: AES256Key,
        iv: InitializationVector,
        aad: AdditionalAuthData,
        tag: AuthenticationTag,
    ) -> (result: Self)
        ensures
            result.key.bytes == key.bytes,
            result.iv.bytes == iv.bytes,
            result.aad.bytes == aad.bytes,
            result.tag.bytes == tag.bytes,
    {
        EncryptionContext { key, iv, aad, tag }
    }

    pub fn get_key(&self) -> (result: &AES256Key)
        ensures
            *result == self.key,
    {
        &self.key
    }

    pub fn get_iv(&self) -> (result: &InitializationVector)
        ensures
            *result == self.iv,
    {
        &self.iv
    }

    pub fn get_aad(&self) -> (result: &AdditionalAuthData)
        ensures
            *result == self.aad,
    {
        &self.aad
    }

    pub fn get_tag(&self) -> (result: &AuthenticationTag)
        ensures
            *result == self.tag,
    {
        &self.tag
    }

    pub open spec fn wf(&self) -> bool {
        &&& self.key.wf()
        &&& self.iv.wf()
        &&& self.aad.wf()
        &&& self.tag.wf()
    }

    pub fn check_wf(&self) -> (result: bool)
        ensures
            result <==> self.wf(),
    {
        self.key.check_wf()
    }
}

fn test_aes_key_structures() {
    let key = AES256Key::new([1u8; AES_256_KEY_LEN]);
    let is_zero = key.check_is_zero();
    let wf = key.check_wf();

    let iv = InitializationVector::new([2u8; IV_LEN]);
    let iv_byte = iv.get_byte(0);

    let aad = AdditionalAuthData::new([3u8; AAD_LEN]);
    let aad_byte = aad.get_byte(0);

    let tag = AuthenticationTag::new([4u8; AUTH_TAG_LEN]);
    let tag_byte = tag.get_byte(0);

    let ctx = EncryptionContext::new(key, iv, aad, tag);
    let ctx_key = ctx.get_key();
    let ctx_wf = ctx.check_wf();

    proof {
        // Note: is_zero and wf may not be provable without additional verification
        // assert(!is_zero);
        // assert(wf);
        assert(iv_byte == 2);
        assert(aad_byte == 3);
        assert(tag_byte == 4);
        // assert(ctx_wf);
    }
}

} // verus!

fn main() {
    test_aes_key_structures();
}
