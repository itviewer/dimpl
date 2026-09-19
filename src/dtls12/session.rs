//!
use std::error::Error;
use std::panic::RefUnwindSafe;

///
pub trait SessionStore: Send + Sync + RefUnwindSafe {
    ///
    fn session_id(&self) -> [u8; 32];
    ///
    fn master_secret(&self, key: &[u8]) -> Result<[u8; 48], Box<dyn Error>>;
}
