use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod poll_program {
    use super::*;

    pub fn create_poll(
        ctx: Context<CreatePoll>,
        _name: String,
        options: u8,
        start: i64,
        end: i64,
        description: String,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        if now > end || end < start {
            return Err(PollError::InvalidDates.into());
        }
        ctx.accounts.poll.set_inner(Poll {
            options,
            start,
            end,
            authority: *ctx.accounts.signer.key,
            bump: ctx.accounts.poll.bump,
            description,
        });
        Ok(())
    }

    pub fn add_option(ctx: Context<AddOption>, _option_number: u8, description: String) -> Result<()> {
        ctx.accounts.option.set_inner(VoteOption {
            count: 0,
            description,
            bump: ctx.bumps.option,
        });
        Ok(())
    }

    pub fn cast_vote(ctx: Context<CastVote>, _cast: u8) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        if now < ctx.accounts.poll.start || now > ctx.accounts.poll.end {
            return Err(PollError::EventClose.into());
        }
        ctx.accounts.option.count = ctx.accounts.option.count.checked_add(1).unwrap();
        emit!(VoteEvent {
            vote_option: ctx.accounts.option.clone().into_inner()
        });
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(_name: String)]
pub struct CreatePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init, seeds = [b"poll", _name.as_bytes()], payer = signer, bump, space = Poll::LEN)]
    pub poll: Account<'info, Poll>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction( _option_number: u8)]
pub struct AddOption<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub poll: Account<'info, Poll>,
    #[account(init, seeds = [b"option", poll.key().as_ref(), &[_option_number]], payer = signer, bump, space = VoteOption::LEN)]
    pub option: Account<'info, VoteOption>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_cast: u8)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [b"option", poll.key().as_ref(), &[_cast]], bump = option.bump)]
    pub option: Account<'info, VoteOption>,
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

#[account]
pub struct Poll {
    pub options: u8,
    pub start: i64,
    pub end: i64,
    pub authority: Pubkey,
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
