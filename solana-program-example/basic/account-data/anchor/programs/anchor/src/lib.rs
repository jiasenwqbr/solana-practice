use anchor_lang::prelude::*;

declare_id!("24HuDqEnLXg2uk1GDG1udfo2qNwBgxKCnzkWxjRTtrnz");

#[program]
pub mod anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
