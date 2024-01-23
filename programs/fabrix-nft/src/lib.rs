use anchor_lang::prelude::*;

declare_id!("HjtErzD75J7kFyjTky7QJUfk8h9iQZignFdbnq1JNXfn");

#[program]
pub mod fabrix_nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
