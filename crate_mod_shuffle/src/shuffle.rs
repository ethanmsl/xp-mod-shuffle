//! Shuffling code

use derive_builder::Builder;
use derive_more::Constructor;

/// Silly struct. Value, what it's modulo, and a forward and reverse (inverse) prime for shuffling
///
#[derive(Clone, Debug, Builder, Constructor)]
// Note: Buidler takes `structnameBUILDER` as struct start, *not* `structname`
pub struct ModShuffler {
    val:                     u64,
    modulo:                  u64,
    prime_forward_shuffler:  u64,
    prime_backward_shuffler: u64,
}

impl Default for ModShuffler {
    fn default() -> Self {
        ModShuffler { val:                     0,
                      modulo:                  32,
                      prime_forward_shuffler:  3,
                      prime_backward_shuffler: 11, }
    }
}
impl ModShuffler {}
