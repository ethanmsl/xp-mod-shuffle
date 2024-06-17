//! Shuffle numbers mod m by some relatively prime number pf and it's inverse pb.

pub mod shuffle;

/// Very early error type that can flow into standard error handling with type coercions.
pub mod error {
    pub type Result<T> = core::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>;
}
