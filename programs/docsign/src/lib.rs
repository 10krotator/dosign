#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("CvQLJvzRYKbua7unJYMvybGtq6eXfUxBqqqdEgusY65i");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod docsign {
    use super::*;

    pub fn set_document(ctx: Context<SetDocument>, document: String, signers: Vec<String>) -> Result<()> {

        msg!("Greetings from: {:?}", ctx.program_id);
        msg!("Setting document: {}", document);
        msg!("Signers: {:?}", signers);
        ctx.accounts.document_account.set_inner(
            Document {
                document,
                document_signers: signers,
            }
        );
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Document {

    #[max_len(50)]
    pub document: String,

    #[max_len(5,50)]
    pub document_signers: Vec<String>,
}

#[derive(Accounts)]
pub struct SetDocument<'info> {

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Document::INIT_SPACE,
        seeds = [b"document", user.key().as_ref()],
        bump,
    )]
    pub document_account: Account<'info, Document>,

    pub system_program: Program<'info, System>,
}
