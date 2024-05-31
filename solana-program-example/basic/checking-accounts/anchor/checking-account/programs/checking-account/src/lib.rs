use anchor_lang::prelude::*;

declare_id!("HCATzJKnmLCt2irJNhJMvtEypz715PzGDEHCFRUAAaKP");

#[program]
pub mod checking_account {
    use super::*;

    pub fn check_account(_ctx: Context<CheckingAccounts>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CheckingAccounts<'info> {
    payer: Signer<'info>,

    #[account(mut)]
    account_to_create: UncheckedAccount<'info>,
    #[account(mut,owner=id())]
    account_to_change: UncheckedAccount<'info>,
    system_program: Program<'info, System>,
}
