/***********
 * Imports *
 ***********/
use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

/**************
 * Program ID *
 **************/
declare_id!("BTw34Cvm8Jh9cNBXpeBM9qCV8nNwTV7vCb7BchMBi8Hc");

/***********
 * Program *
 ***********/
// Macro
#[program]
// Module
pub mod gifportal {
    use super::*;

    pub fn start_stuff_off(context: Context<StartStuffOff>) -> ProgramResult {
        let base_account = &mut context.accounts.base_account;

        base_account.total_gifs = 0;

        Ok(())
    }

    pub fn add_gif(context: Context<AddGif>, gif_link: String)
        -> ProgramResult {
            let base_account = &mut context.accounts.base_account;
            let user = &mut context.accounts.user;

            let item = ItemStruct {
                gif_link: gif_link.to_string(),
                user_address: *user.to_account_info().key,
            };

            base_account.gif_list.push(item);

            base_account.total_gifs += 1;

            Ok(())
        }
}

/**********************
 * Structs (Contexts) *
 **********************/
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer=user, space=9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>
}
