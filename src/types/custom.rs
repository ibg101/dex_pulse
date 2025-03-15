#[derive(Debug, Clone, Copy)]
pub enum Dex {
    Raydium,
    Meteora
}

// ---- my custom token meta ----
#[derive(Default, Debug)]
pub struct TokenMeta {
    pub signers: Vec<String>,
    pub base: SharedTokenMeta,
    pub quote: SharedTokenMeta
}

#[derive(Default, Debug)]
pub struct SharedTokenMeta {
    pub mint: String,
    pub vault: String,  // Pool 1 / Pool 2
    pub added_liq_amount: u64,
    pub decimals: u8
}

impl TokenMeta {
    pub fn default_preallocated() -> Self {
        const PUBKEY_LEN: usize = 32;

        let base: SharedTokenMeta = SharedTokenMeta {
            mint: String::with_capacity(PUBKEY_LEN),
            vault: String::with_capacity(PUBKEY_LEN),
            ..Default::default()
        };

        let quote: SharedTokenMeta = SharedTokenMeta {
            mint: String::with_capacity(PUBKEY_LEN),
            vault: String::with_capacity(PUBKEY_LEN),
            ..Default::default()
        };

        Self { base, quote, ..Default::default() }
    }
}
// ---- my custom token meta ----