use anchor_lang::prelude::*;

declare_id!("8iZz65rU8qKdEL9TmUgCkVJEpcWotiEU8315fw6NsdDN");

#[program]
pub mod solana_hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
