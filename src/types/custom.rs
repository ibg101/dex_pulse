use super::rpc::LoadedAddresses;


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

#[derive(Debug)]
pub struct AccountKeys<'a> {
    static_keys: &'a [String],
    dynamic_keys: Option<&'a LoadedAddresses>
}

// syntactic sugar (allows using account_keys[i] instead of account_keys.get(i).unwrap())
impl std::ops::Index<usize> for AccountKeys<'_> {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        &self.get(index).expect("invalid index!")
    }
}

impl<'a> AccountKeys<'a> {
    pub fn new(static_keys: &'a [String], dynamic_keys: Option<&'a LoadedAddresses>) -> Self {
        Self { static_keys, dynamic_keys }
    }

    fn segments_iter(&self) -> impl Iterator<Item = &[String]> {
        let segments_collection: [&[String]; 3] = if let Some(dyn_keys) = self.dynamic_keys {
            [self.static_keys, &dyn_keys.writable, &dyn_keys.readonly]
        } else {
            [self.static_keys, &[], &[]]
        };
        segments_collection.into_iter()
    }
    
    pub fn get(&self, mut index: usize) -> Option<&String> {
        for segment in self.segments_iter() {
            let segment_len: usize = segment.len();
            if segment_len > index {
                return Some(&segment[index]);
            }
            index = index.saturating_sub(segment_len);  // using -= in order to work with new segment from proper index
        }
        None
    } 
}