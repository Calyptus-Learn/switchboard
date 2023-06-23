use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;
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
