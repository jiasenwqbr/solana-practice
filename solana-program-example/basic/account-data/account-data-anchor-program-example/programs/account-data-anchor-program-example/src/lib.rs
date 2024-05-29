use anchor_lang::prelude::*;

declare_id!("hB93vXZbZQ42Zczn2joFoVtmVw3ZG87wLrVDtCkiTYP");

#[program]
pub mod account_data_anchor_program_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
