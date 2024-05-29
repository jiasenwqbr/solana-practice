#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;
use instructions::*;
pub mod instructions;
pub mod state;

declare_id!("hB93vXZbZQ42Zczn2joFoVtmVw3ZG87wLrVDtCkiTYP");

#[program]
pub mod account_data_anchor_program_example {
    use instructions::create;

    use super::*;

    pub fn anchor_program_example(
        ctx: Context<CreateAddressInfo>,
        name: String,
        house_number: u8,
        city: String,
        street: String,
    ) -> Result<()> {
        create::create_address_info(ctx, name, house_number, street, city)
    }
}
