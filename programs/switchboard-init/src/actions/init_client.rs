use crate::*;

#[derive(Accounts)]
#[instruction(params: InitClientParams)]
pub struct InitClient {}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InitClientParams {}

impl InitClient {
    pub fn validate(&self, _ctx: &Context<Self>, param: &InitClientParams) -> Result<()> {
        msg!("init_client validate");
        Ok(())
    }

    pub fn actuate(ctx: &Context<Self>, params: &InitClientParams) -> Result<()> {
        msg!("init_client actuate");
        Ok(())
    }
}