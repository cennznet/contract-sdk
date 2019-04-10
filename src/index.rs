//!
//! An index of CENNZnet reference types and constants
//!

/// Mappings to CENNZnet runtime types
pub mod types {
    use primitives::H256;

    // CENNZnet type mappings
    pub type AccountId = H256;
    pub type AssetId = u32;
    pub type Balance = u64;
    pub type Timestamp = u64;
}

/// Mappings to CENNZnet asset IDs
pub mod asset {
    pub const CENNZ: u32 = 16_000;
    pub const CENTRAPAY: u32 = 16_001;
}
