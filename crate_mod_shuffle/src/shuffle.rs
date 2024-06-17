//! Shuffling code

use derive_builder::Builder;
use derive_more::Constructor;

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct ModShuffler {
    val:                     u64,
    modulo:                  u64,
    prime_forward_shuffler:  u64,
    prime_backward_shuffler: u64,
}

/// Generated via validation of ProposedModShuffler (I should really wrap into the build method)
impl ModShuffler {
    pub fn fwd_shuffle(&self) -> Self {
        // TODO: rel cost of 128bit cating and action, vs testing and contitionally running
        let mut v = self.val as u128;
        let p = self.prime_forward_shuffler as u128;
        let m = self.modulo as u128;

        let v = ((v * p) % m) as u64;

        let mut out = self.clone();
        out.val = v;
        out
    }

    pub fn rev_shuffle(&self) -> Self {
        // TODO: rel cost of 128bit cating and action, vs testing and contitionally running
        let mut v = self.val as u128;
        let r = self.prime_backward_shuffler as u128;
        let m = self.modulo as u128;

        let v = ((v * r) % m) as u64;

        let mut out = self.clone();
        out.val = v;
        out
    }
}

/// Silly struct. Value, what it's modulo, and a forward and reverse (inverse) prime for shuffling
///
#[derive(Clone, Debug, Builder, Constructor)]
// Note: Buidler takes `structnameBUILDER` as struct start, *not* `structname`
pub struct ProposedModShuffler {
    val:                     u64,
    modulo:                  u64,
    prime_forward_shuffler:  u64,
    prime_backward_shuffler: u64,
}

impl Default for ProposedModShuffler {
    fn default() -> Self {
        ProposedModShuffler { val:                     0,
                              modulo:                  32,
                              prime_forward_shuffler:  3,
                              prime_backward_shuffler: 11, }
    }
}

impl ProposedModShuffler {
    pub fn validate(&self) -> Result<ModShuffler, Error> {
        Ok(ModShuffler { val:                     self.val,
                         modulo:                  self.modulo,
                         prime_forward_shuffler:  self.prime_forward_shuffler,
                         prime_backward_shuffler: self.prime_backward_shuffler, })
    }
}
