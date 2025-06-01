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

// pub fn unpack_key(data: &[u8]) -> Result<(String, &[u8]), Error> {
//     let key: String = data
//         .get(..32)
//         .map(|slice| bs58::encode(slice).into_string())
//         .ok_or(Error::InvalidInstruction)?;

//     Ok((key, &data[32..]))
// }

pub fn unpack_u16(data: &[u8]) -> Result<(u16, &[u8]), Error> {
    let amount: u16 = data
        .get(..2)
        .and_then(|slice| slice.try_into().ok())
        .map(u16::from_le_bytes)
        .ok_or(Error::InvalidInstruction)?;
    
    Ok((amount, &data[2..]))
}

pub fn unpack_u64(data: &[u8]) -> Result<(u64, &[u8]), Error> {
    let amount: u64 = data
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(Error::InvalidInstruction)?;
    
    Ok((amount, &data[8..]))
}

pub fn unpack_amount_and_decimals(data: &[u8]) -> Result<(u64, u8, &[u8]), Error> {
    let (amount, rest) = unpack_u64(data)?;
    let (&decimals, rest) = rest.split_first().ok_or(Error::InvalidInstruction)?;
    Ok((amount, decimals, rest))
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