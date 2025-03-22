use crate::types::error::Error;


pub fn unpack_option_key(key: [u8; 36]) -> Result<Option<String>, Error> {
    let (tag, pubkey) = key.split_at(4);
    match *tag {
        [0, 0, 0, 0] => Ok(None),
        [1, 0, 0, 0] => Ok(Some(bs58::encode(pubkey).into_string())),
        _ => {
            log::error!("Invalid Pubkey Tag!");
            Err(Error::UnpackOptionKey)
        }
    }
}

pub fn try_array_from_slice<const L: usize>(d: &[u8], start: usize, end: usize) -> Result<[u8; L], Error> {
    d[start..end].try_into().map_err(|e| {
        log::error!("Caused an error: {e}");
        Error::ArrayFromSlice
    })
}

pub fn validate_instruction_accounts_len(instruction_accounts: &[usize], min_expected_len: usize) -> Result<(), Error> {
    // using < instead of != in order to handle cases with multiple signers
    if instruction_accounts.len() < min_expected_len { return Err(Error::ParseInstruction); }
    Ok(())
}