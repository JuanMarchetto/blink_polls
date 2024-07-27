use anchor_lang::prelude::*;
declare_id!("9ukqpC44ttCKHvbMtVSVJ169fFGL28eWKmDSe8v7dytP");

#[program]
pub mod poll_program {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        _name: String,
        options: u8,
        start: i64,
        end: i64,
        description: String,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        if now > ctx.accounts.poll.end || ctx.accounts.poll.end < ctx.accounts.poll.start {
            return Err(PollError::InvalidDates.into());
        }
        ctx.accounts.poll.set_inner(Poll {
            authority: *ctx.accounts.signer.key,
            options,
            start,
            end,
            description,
            bump: ctx.accounts.poll.bump,
        });
        Ok(())
    }

    pub fn add_option(ctx: Context<AddOption>, _option: u8, description: String) -> Result<()> {
        ctx.accounts.option_pda.set_inner(VoteOption {
            count: 0,
            description,
            bump: ctx.accounts.option_pda.bump,
        });
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, _vote: u8) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        if now < ctx.accounts.poll.start || now > ctx.accounts.poll.end {
            return Err(PollError::EventClose.into());
        }
        ctx.accounts.option_pda.count = ctx.accounts.option_pda.count.checked_add(1).unwrap();
        emit!(VoteEvent {
            vote_option: ctx.accounts.option_pda.clone().into_inner()
        });
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(_name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init, seeds = [b"poll", _name.as_bytes()], payer = signer, bump, space = Poll::LEN)]
    pub poll: Account<'info, Poll>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_vote: u8)]
pub struct Vote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [b"option", poll.key().as_ref(), &[_vote]], bump = option_pda.bump)]
    pub option_pda: Account<'info, VoteOption>,
    #[account(
        init,
        payer = signer,
        seeds = [b"lock".as_ref(), poll.key().as_ref(), signer.key().as_ref(),],
        bump,
        space =  8 + 1,
    )]
    pub lock: Account<'info, Lock>,
    #[account(mut)]
    pub poll: Account<'info, Poll>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction( _option: u8)]
pub struct AddOption<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub poll: Account<'info, Poll>,
    #[account(init, seeds = [b"option", poll.key().as_ref(), &[_option]], payer = signer, bump, space = VoteOption::LEN)]
    pub option_pda: Account<'info, VoteOption>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Poll {
    pub authority: Pubkey,
    pub options: u8,
    pub start: i64,
    pub end: i64,
    pub bump: u8,
    pub description: String,
}

impl Poll {
    const LEN: usize = 300;
}

#[account]
pub struct VoteOption {
    pub count: u128,
    pub bump: u8,
    pub description: String,
}

impl VoteOption {
    const LEN: usize = 300;
}

#[account]
#[derive(InitSpace)]
pub struct Lock {
    pub bump: u8,
}

#[event]
pub struct VoteEvent {
    pub vote_option: VoteOption,
}

#[error_code]
pub enum PollError {
    #[msg("The Event is not open.")]
    EventClose,
    #[msg("The Dates are invalid.")]
    InvalidDates,
}
