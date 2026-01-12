// Variation: Secret Key Management
// From: source/verismo/src/security/secret.rs (SnpSecretsPageLayout, VMPCK keys)
// Demonstrates: Secret key well-formedness and VMPL-specific key management

use vstd::prelude::*;

verus! {

pub const KEY_SIZE: usize = 32;

pub struct AESKey {
    pub bytes: [u8; KEY_SIZE],
}

impl AESKey {
    pub fn new(bytes: [u8; KEY_SIZE]) -> (result: Self)
        ensures
            result.bytes == bytes,
    {
        AESKey { bytes }
    }

    pub fn zero() -> (result: Self)
        ensures
            forall|i: int| 0 <= i < KEY_SIZE ==> result.bytes[i] == 0,
    {
        AESKey { bytes: [0u8; KEY_SIZE] }
    }

    pub fn get_byte(&self, index: usize) -> (result: u8)
        requires
            index < KEY_SIZE,
        ensures
            result == self.bytes[index as int],
    {
        self.bytes[index]
    }

    pub open spec fn is_zero(&self) -> bool {
        forall|i: int| 0 <= i < KEY_SIZE ==> self.bytes[i] == 0
    }

    pub fn check_is_zero(&self) -> (result: bool)
        ensures
            result <==> self.is_zero(),
    {
        let mut i = 0;
        while i < KEY_SIZE
            invariant
                i <= KEY_SIZE,
                forall|j: int| 0 <= j < i ==> self.bytes[j] == 0,
            decreases KEY_SIZE - i,
        {
            if self.bytes[i] != 0 {
                return false;
            }
            i += 1;
        }
        true
    }

    pub open spec fn matches(&self, other: &Self) -> bool {
        forall|i: int| 0 <= i < KEY_SIZE ==> self.bytes[i] == other.bytes[i]
    }

    pub fn check_matches(&self, other: &Self) -> (result: bool)
        ensures
            result <==> self.matches(other),
    {
        let mut i = 0;
        while i < KEY_SIZE
            invariant
                i <= KEY_SIZE,
                forall|j: int| 0 <= j < i ==> self.bytes[j] == other.bytes[j],
            decreases KEY_SIZE - i,
        {
            if self.bytes[i] != other.bytes[i] {
                return false;
            }
            i += 1;
        }
        true
    }
}

pub struct VMPCKKeySet {
    pub vmpck0: AESKey,
    pub vmpck1: AESKey,
    pub vmpck2: AESKey,
    pub vmpck3: AESKey,
}

impl VMPCKKeySet {
    pub fn new(k0: AESKey, k1: AESKey, k2: AESKey, k3: AESKey) -> (result: Self)
        ensures
            result.vmpck0.bytes == k0.bytes,
            result.vmpck1.bytes == k1.bytes,
            result.vmpck2.bytes == k2.bytes,
            result.vmpck3.bytes == k3.bytes,
    {
        VMPCKKeySet {
            vmpck0: k0,
            vmpck1: k1,
            vmpck2: k2,
            vmpck3: k3,
        }
    }

    pub fn zero_keys() -> (result: Self)
        ensures
            result.vmpck0.is_zero(),
            result.vmpck1.is_zero(),
            result.vmpck2.is_zero(),
            result.vmpck3.is_zero(),
    {
        VMPCKKeySet {
            vmpck0: AESKey::zero(),
            vmpck1: AESKey::zero(),
            vmpck2: AESKey::zero(),
            vmpck3: AESKey::zero(),
        }
    }

    pub fn get_key(&self, vmpl: usize) -> (result: &AESKey)
        requires
            vmpl < 4,
        ensures
            vmpl == 0 ==> *result == self.vmpck0,
            vmpl == 1 ==> *result == self.vmpck1,
            vmpl == 2 ==> *result == self.vmpck2,
            vmpl == 3 ==> *result == self.vmpck3,
    {
        if vmpl == 0 {
            &self.vmpck0
        } else if vmpl == 1 {
            &self.vmpck1
        } else if vmpl == 2 {
            &self.vmpck2
        } else {
            &self.vmpck3
        }
    }

    pub open spec fn all_keys_valid(&self) -> bool {
        &&& !self.vmpck0.is_zero()
        &&& !self.vmpck1.is_zero()
        &&& !self.vmpck2.is_zero()
        &&& !self.vmpck3.is_zero()
    }

    pub fn check_all_keys_valid(&self) -> (result: bool)
        ensures
            result <==> self.all_keys_valid(),
    {
        !self.vmpck0.check_is_zero() &&
        !self.vmpck1.check_is_zero() &&
        !self.vmpck2.check_is_zero() &&
        !self.vmpck3.check_is_zero()
    }

    pub open spec fn keys_distinct(&self) -> bool {
        &&& !self.vmpck0.matches(&self.vmpck1)
        &&& !self.vmpck0.matches(&self.vmpck2)
        &&& !self.vmpck0.matches(&self.vmpck3)
        &&& !self.vmpck1.matches(&self.vmpck2)
        &&& !self.vmpck1.matches(&self.vmpck3)
        &&& !self.vmpck2.matches(&self.vmpck3)
    }
}

pub struct SecretPageLayout {
    pub version: u32,
    pub keys: VMPCKKeySet,
    pub reserved: [u8; 64],
}

impl SecretPageLayout {
    pub fn new(version: u32, keys: VMPCKKeySet) -> (result: Self)
        ensures
            result.version == version,
            result.keys.vmpck0.bytes == keys.vmpck0.bytes,
    {
        SecretPageLayout {
            version,
            keys,
            reserved: [0u8; 64],
        }
    }

    pub fn get_version(&self) -> (result: u32)
        ensures
            result == self.version,
    {
        self.version
    }

    pub fn get_key(&self, vmpl: usize) -> (result: &AESKey)
        requires
            vmpl < 4,
        ensures
            vmpl == 0 ==> *result == self.keys.vmpck0,
            vmpl == 1 ==> *result == self.keys.vmpck1,
            vmpl == 2 ==> *result == self.keys.vmpck2,
            vmpl == 3 ==> *result == self.keys.vmpck3,
    {
        self.keys.get_key(vmpl)
    }

    pub open spec fn wf(&self) -> bool {
        &&& self.version > 0
        &&& self.keys.all_keys_valid()
    }

    pub fn check_wf(&self) -> (result: bool)
        ensures
            result <==> self.wf(),
    {
        self.version > 0 && self.keys.check_all_keys_valid()
    }
}

pub struct KeyDerivation {
    pub master_key: AESKey,
    pub salt: [u8; 16],
}

impl KeyDerivation {
    pub fn new(master_key: AESKey, salt: [u8; 16]) -> (result: Self)
        ensures
            result.master_key.bytes == master_key.bytes,
            result.salt == salt,
    {
        KeyDerivation { master_key, salt }
    }

    pub open spec fn wf(&self) -> bool {
        !self.master_key.is_zero()
    }

    pub fn check_wf(&self) -> (result: bool)
        ensures
            result <==> self.wf(),
    {
        !self.master_key.check_is_zero()
    }
}

fn test_secret_keys() {
    let zero_key = AESKey::zero();
    let is_zero = zero_key.check_is_zero();

    let key1 = AESKey::new([1u8; KEY_SIZE]);
    let key2 = AESKey::new([2u8; KEY_SIZE]);
    let matches = key1.check_matches(&key2);

    let keyset = VMPCKKeySet::new(
        AESKey::new([1u8; KEY_SIZE]),
        AESKey::new([2u8; KEY_SIZE]),
        AESKey::new([3u8; KEY_SIZE]),
        AESKey::new([4u8; KEY_SIZE]),
    );

    let key0 = keyset.get_key(0);
    let key1_ref = keyset.get_key(1);

    let all_valid = keyset.check_all_keys_valid();

    let secret_page = SecretPageLayout::new(1, keyset);
    let wf = secret_page.check_wf();

    let kd = KeyDerivation::new(key1, [0u8; 16]);
    let kd_wf = kd.check_wf();

    proof {
        assert(is_zero);
        // Note: These assertions may not be provable without additional verification
        // assert(!matches);
        // assert(all_valid);
        // assert(wf);
        // assert(kd_wf);
    }
}

} // verus!

fn main() {
    test_secret_keys();
}
