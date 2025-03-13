use borsh::{BorshDeserialize, BorshSerialize};


const MAX_BIN_PER_POSITION: usize = 70;
const NUM_REWARDS: usize = 2; 


// ---- meteora modify liquidity ----
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct ModifyLiquidity {
    pub position: PositionV2,
    pub lb_pair: [u8; 32],
    bin_array_bitmap_extension: Option<[u8; 32]>,
    pub user_token_x: [u8; 32],  // base ata
    pub user_token_y: [u8; 32],
    pub reserve_x: [u8; 32],     // base vault
    pub reserve_y: [u8; 32],
    pub token_x_mint: [u8; 32],  // base mint
    pub token_y_mint: [u8; 32],
    bin_array_lower: [u8; 32],
    bin_array_upper: [u8; 32],
    sender: [u8; 32],  // signer
    token_x_program: [u8; 32],  // token programs
    token_y_program: [u8; 32],

    event_authority: Option<[u8; 32]>,
    program: Option<[u8; 32]>
}
// ---- meteora modify liquidity ----


#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PositionV2 {
    /// The LB pair of this position
    pub lb_pair: [u8; 32],
    /// Owner of the position. Client rely on this to to fetch their positions.
    pub owner: [u8; 32],
    /// Liquidity shares of this position in bins (lower_bin_id <-> upper_bin_id). This is the same as LP concept.
    pub liquidity_shares: [u128; MAX_BIN_PER_POSITION],
    /// Farming reward information
    pub reward_infos: [UserRewardInfo; MAX_BIN_PER_POSITION],
    /// Swap fee to claim information
    pub fee_infos: [FeeInfo; MAX_BIN_PER_POSITION],
    /// Lower bin ID
    pub lower_bin_id: i32,
    /// Upper bin ID
    pub upper_bin_id: i32,
    /// Last updated timestamp
    pub last_updated_at: i64,
    /// Total claimed token fee X
    pub total_claimed_fee_x_amount: u64,
    /// Total claimed token fee Y
    pub total_claimed_fee_y_amount: u64,
    /// Total claimed rewards
    pub total_claimed_rewards: [u64; 2],
    /// Operator of position
    pub operator: [u8; 32],
    /// Time point which the locked liquidity can be withdraw
    pub lock_release_point: u64,
    /// _padding_0, previous subjected_to_bootstrap_liquidity_locking, BE CAREFUL FOR TOMBSTONE WHEN REUSE !!
    pub _padding_0: u8,
    /// Address is able to claim fee in this position, only valid for bootstrap_liquidity_position
    pub fee_owner: [u8; 32],
    /// Reserved space for future use
    pub _reserved: [u8; 87],
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct UserRewardInfo {
    pub reward_per_token_completes: [u128; NUM_REWARDS],
    pub reward_pendings: [u64; NUM_REWARDS],
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct FeeInfo {
    pub fee_x_per_token_complete: u128,
    pub fee_y_per_token_complete: u128,
    pub fee_x_pending: u64,
    pub fee_y_pending: u64,
}