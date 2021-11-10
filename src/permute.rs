use self::private::PermuteKeyPrivate;
use crate::usize_generator::PermuteKeyData;

mod private {
    use crate::usize_generator::{PermuteKeyData, UsizeGenerator};

    pub trait PermuteKeyPrivate {
        fn make_usize_random_generator(self, len: u64) -> UsizeGenerator;
    }

    impl<'a> PermuteKeyPrivate for &'a [u8] {
        fn make_usize_random_generator(self, len: u64) -> UsizeGenerator {
            let mut rand_builder = PermuteKeyData::new();
            rand_builder.add_bytes(self);
            rand_builder.make_usize_random_generator(len)
        }
    }

    impl<'a> PermuteKeyPrivate for &'a mut [u8] {
        fn make_usize_random_generator(self, len: u64) -> UsizeGenerator {
            (&*self).make_usize_random_generator(len)
        }
    }

    impl PermuteKeyPrivate for PermuteKeyData {
        fn make_usize_random_generator(mut self, len: u64) -> UsizeGenerator {
            // NOTE: We add in the length to the RandomData in order to address
            // a subtle issue. Without these bytes, the permutation of a list
            // of length 10 is going to look very similar to the permutation of
            // a list of length 11 when using the same RandomData. But, critically
            // not identical - and this could cause a user to think there there is
            // no relationship. I'm not 100% sure how this could be reasonably exploited -
            // but its easy enough to avoid altogether.
            self.add_bytes(&len.to_le_bytes());
            self.into_usize_generator()
        }
    }
}

/// A type must implement PermuteKey to be used as a `permute_key` value
/// for the [`permute`] function.
///
/// This trait is currently Sealed.
pub trait PermuteKey: PermuteKeyPrivate {}

impl<'a> PermuteKey for &'a [u8] {}
impl<'a> PermuteKey for &'a mut [u8] {}
impl PermuteKey for PermuteKeyData {}

/// Permute the items in `data` using a permutation determined
/// by the given `permute_key`.
///
/// `permute_key` can either by a byte slice (`&[u8]` or `&mut [u8]`) or an
/// instance of [`RandomData`]. As long as this value is unpredictable, the
/// permutation will also be unpredictable. The same `permute_key` value will
/// always result in the same permutation.
///
/// The items of `data` are permuted in place.
pub fn permute<K: PermuteKey, T>(permute_key: K, data: &mut [T]) {
    // NOTE: The case to u64 is valid because we check in lib.rs that u64 must have
    //       at least as many bits as usize or we won't compile.
    let mut rand = permute_key.make_usize_random_generator(data.len() as u64);

    for (idx, max_swap_offset) in (1..data.len()).rev().enumerate() {
        let swap_offset = rand.next_usize_max(max_swap_offset);
        data.swap(idx, idx + swap_offset);
    }
}
