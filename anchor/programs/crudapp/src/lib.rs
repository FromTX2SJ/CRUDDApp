#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("8gs5AjZpsyLZfTeiTe5TdSyuzeaxQ1GZWEM6CqCWifYv");

#[program]
pub mod crudapp {
    use super::*;

    pub fn create_journal_entry(ctx: Context<CreateEntry>, title: String, message: String) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry_state;
        journal_entry.owner = ctx.accounts.owner.key();
        journal_entry.title = title;
        journal_entry.message = message;
        Ok(())
    }

    pub fn update_journal_entry(ctx: Context<UpdateEntry>, _title: String, message: String) -> Result<()> {
      let journal_entry = &mut ctx.accounts.journal_entry_state;
      journal_entry.message = message;
      Ok(())
    }

    pub fn delete_journal_entry(_ctx: Context<DeleteEntry>, _title: String) -> Result<()> {
      Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateEntry<'info> {
   #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
      init,
      payer = owner,
      space = 8 + JounernalEntryState::INIT_SPACE, 
      seeds=[title.as_bytes(), owner.key().as_ref()], 
      bump
    )]
    pub journal_entry_state: Account<'info, JounernalEntryState>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateEntry<'info> {
  #[account(mut)]
  pub owner: Signer<'info>,

  #[account(
    mut, 
    seeds = [title.as_bytes(), owner.key().as_ref()], 
    bump,
    realloc = 8 + JounernalEntryState::INIT_SPACE,
    realloc::payer = owner, 
    realloc::zero = true,
  )]
  pub journal_entry_state: Account<'info, JounernalEntryState>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteEntry<'info> {
  #[account(mut)]
  pub owner: Signer<'info>,

  #[account(
    mut,
    seeds = [title.as_bytes(), owner.key().as_ref()], 
    bump,
    close =owner,
  )] 
  pub journal_entry_state: Account<'info, JounernalEntryState>,

  pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct JounernalEntryState {
    pub owner: Pubkey,

    #[max_len(50)]
    pub title: String,

    #[max_len(1000)]
    pub message: String,
}
