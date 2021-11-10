use chacha20::{
    cipher::{NewCipher, StreamCipher},
    ChaCha20,
};
use sha2::{self, digest::FixedOutput, Digest, Sha256};

/// `PermuteKeyData` can be used to incrementally collect data to form a key
/// to pass to [`permute`](crate::permute).
#[derive(Debug, Clone)]
pub struct PermuteKeyData {
    sha: Sha256,
}

impl PermuteKeyData {
    /// Create a new `PermuteKeyData` value
    pub fn new() -> PermuteKeyData {
        PermuteKeyData { sha: Sha256::new() }
    }

    /// Provide some input data
    pub fn add_bytes(&mut self, bytes: &[u8]) {
        Digest::update(&mut self.sha, bytes);
    }

    pub(crate) fn into_usize_generator(self) -> UsizeGenerator {
        let key = self.sha.finalize_fixed();
        let nonce = [0u8; 12];
        let chacha = ChaCha20::new(&key, &nonce.into());
        UsizeGenerator { chacha }
    }
}

pub struct UsizeGenerator {
    chacha: ChaCha20,
}

impl UsizeGenerator {
    // Get a random u64 value without any limit on its value
    fn next_u64(&mut self) -> u64 {
        let mut tmp = [0u8; 8];
        self.chacha.apply_keystream(&mut tmp);
        u64::from_le_bytes(tmp)
    }

    // Get a random u64 value, less than or equal to max
    fn next_u64_max(&mut self, max: u64) -> u64 {
        // Strategy: we generate random u64 values in a loop until
        // we find one that is less than or equal to max. We return
        // the first one we find. We avoid using a mod operator because
        // we want to make sure that all output values are equally likely.
        //
        // Details:
        //
        // Just randomly generating u64 values would be inefficient
        // for small values of max since it would take a *very* long
        // time to find a small enough one. However, we still want to
        // make sure that all output values are equally likely. We accomplish
        // this by truncating the u64 values down to the smallest number
        // of bits required to hold the specified max value. Then, we
        // test the value to see if its less than or equal to max - meaning
        // that around half the time we will be able to return the value.
        //
        // The truncation of the u64 value is accomplished by applying a
        // bitmask to the full size random u64 value. The mask needs to return
        // the smallest number of bits required to hold the max value. Some
        // examples:
        //
        // * max 6 - we need a mask of 0x7 since 6 requires 3 bits.
        // * max 7 - we need a mask of 0x7 since 7 requires 3 bits.
        // * max 8 - we need a mask of 0xf since 8 requires 4 bits.
        //
        // The pattern to note: we can calculate the mask value by
        // finding the next power of two _greater_ than the max value
        // and then subtracting 1.
        //
        // NOTE: We don't special case the case where max == 0 since it doesn't
        // seem worth the cost of an extra branch.

        let mask = if max >= (1 << 63) {
            // If the top bit is set, we want a mask with all bits set
            u64::MAX
        } else {
            // Examples:
            // * if max is 6, we need to generate random numbers in range 0-7
            //   until we find one 6 or less which we can return
            // * if max is 7, we need to generate random numbers in range 0-7
            //   and we can return whatever we generate
            // * if max is 8, we need to generate random numbers in range 0-15
            //   until we find one 8 or less which we can return
            // So, we have to find the "next_power_of_two()" of (max + 1)
            // since next_power_of_two() returns the value itself if it is already
            // a power of two.
            (max + 1).next_power_of_two() - 1
        };
        loop {
            let rand_val = self.next_u64() & mask;
            if rand_val <= max {
                return rand_val;
            }
        }
    }

    pub fn next_usize_max(&mut self, max: usize) -> usize {
        // NOTE: The case to u64 is valid because we check in lib.rs that u64 must have
        //       at least as many bits as usize or we won't compile. The case back to usize
        //       is valid since the returned value cannot be any larger than max - which
        //       was itself passed in as a usize.
        self.next_u64_max(max as u64) as usize
    }
}
