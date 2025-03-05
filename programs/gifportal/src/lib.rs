/***********
 * Imports *
 ***********/
use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use uuid::Uuid;

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

    pub fn initialize(context: Context<Initialize>) -> ProgramResult {
        let gifs_account = &mut context.accounts.gifs_account;

        gifs_account.gif_count = 0;

        Ok(())
    }

    pub fn create_gif(context: Context<CreateGif>, url: String)
        -> ProgramResult {
            let gifs_account = &mut context.accounts.gifs_account;
            let user = &mut context.accounts.user;

            let id: String = Uuid::new_v4().to_string();
            let gif = Gif {
                id,
                url: url.to_string(),
                user_address: *user.to_account_info().key,
            };

            gifs_account.gif_count += 1;
            gifs_account.gifs.push(gif);

            Ok(())
        }
}

/**********************
 * Structs (Contexts) *
 **********************/
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer=user, space=9000)]
    pub gifs_account: Account<'info, GifsAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GifsAccount {
    pub gif_count: u64,
    pub gifs: Vec<Gif>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Gif {
    pub id: String,
    pub url: String,
    pub user_address: Pubkey,
}

#[derive(Accounts)]
pub struct CreateGif<'info> {
    #[account(mut)]
    pub gifs_account: Account<'info, GifsAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}
