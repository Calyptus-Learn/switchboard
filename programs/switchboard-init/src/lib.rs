use anchor_lang::prelude::*;

declare_id!("5F1aAkVdT4M4b2XcBAtuMWKXRM9niTTnqg3TzAWZeANo");

#[program]
pub mod switchboard_init {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let switchboard = &mut ctx.accounts.switchboard;
        switchboard.authority = *ctx.accounts.user.key;
        switchboard.data = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
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
