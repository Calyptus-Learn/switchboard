use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;
pub use anchor_spl::token::{Token, TokenAccount};
use std::convert::TryInto;
use switchboard_v2::{
    AggregatorAccountData, OracleQueueAccountData, PermissionAccountData, SbState,
    SwitchboardDecimal, VrfAccountData, VrfRequestRandomness, SWITCHBOARD_PROGRAM_ID,
};

pub mod actions;
pub use actions::*;

declare_id!("91ZY5HgJnypFcnzkYqMU8M4V5UMrvYGXgQPtZ1cHaj2c");

#[program]
pub mod switchboard_init {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let switchboard = &mut ctx.accounts.switchboard;
        switchboard.authority = *ctx.accounts.user.key;
        switchboard.data = 0;
        Ok(())
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn init_client(ctx: Context<InitClient>, params: InitClientParams) -> Result<()> {
        InitClient::actuate(&ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn request_randomness(
        ctx: Context<RequestRandomness>,
        params: RequestRandomnessParams,
    ) -> Result<()> {
        RequestRandomness::actuate(&ctx, &params)
    }
    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn consume_randomness(
        ctx: Context<ConsumeRandomness>,
        params: ConsumeRandomnessParams,
    ) -> Result<()> {
        ConsumeRandomness::actuate(&ctx, &params)
    }
}

const STATE_SEED: &[u8] = b"CLIENTSEED";

#[repr(packed)]
#[account(zero_copy(unsafe))]
#[derive(Default)]
pub struct VrfClientState {
    pub bump: u8,
    pub max_result: u64,
    pub result: u128,
    pub result_buffer: [u8; 32],
    pub timestamp: i64,
    pub vrf: Pubkey,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8)]
    pub switchboard: Account<'info, Switchboard>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Switchboard {
    pub authority: Pubkey,
    pub data: u64,
}

#[derive(Accounts)]
#[instruction(params: ReadResultParams)]
pub struct ReadResults<'info> {
    pub aggregator: AccountLoader<'info, AggregatorAccountData>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ReadResultParams {
    pub max_confidence_interval: Option<f64>,
}

#[error_code]
#[derive(Eq, PartialEq)]
pub enum FeedErrorCode {
    #[msg("Not a valid switchboard account")]
    InvalidSwitchboardAccount,
    #[msg("Switchboard feed has not been updated in 5 minutes")]
    StaleFeed,
    #[msg("Switchboard feed exceeded confidence interval")]
    ConfidenceIntervalExceeded,
}

#[error_code]
#[derive(Eq, PartialEq)]
pub enum VrfClientErrorCode {
    #[msg("Switchboard VRF Account's authority should be set to the client's state pubkey")]
    InvalidVrfAuthorityError,
    #[msg("The max result must not exceed u64")]
    MaxResultExceedsMaximum,
    #[msg("Invalid VRF account provided")]
    InvalidVrfAccount,
    #[msg("Not a valid Switchboard account")]
    InvalidSwitchboardAccount,
}

#[event]
pub struct VrfClientCreated {
    pub vrf_client: Pubkey,
    pub max_result: u64,
    pub timestamp: i64,
}

#[event]
pub struct RandomnessRequested {
    pub vrf_client: Pubkey,
    pub max_result: u64,
    pub timestamp: i64,
}

#[event]
pub struct VrfClientUpdated {
    pub vrf_client: Pubkey,
    pub max_result: u64,
    pub result_buffer: [u8; 32],
    pub result: u128,
    pub timestamp: i64,
}
