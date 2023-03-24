use anchor_lang::prelude::*;
use anchor_spl::{token::{self, Mint, Token, TokenAccount}, associated_token::AssociatedToken};

declare_id!("G2QagUQXZRktxGae3diNtJhA592W93mBVk5Hy3Usnrqi");

#[program]
pub mod anchor_movie_review {

    use std::result;

    use anchor_lang::solana_program::program::invoke_signed;
    use mpl_token_metadata::instruction::create_metadata_accounts_v3;

    use super::*;

    pub fn add_movie_review(ctx: Context<AddMovieReview>, title: String, description: String, rating: u8) -> Result<()> {

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.title = title;
        movie_review.description = description;

        if rating > 5 || rating < 1 {
            msg!("Ratings are invalid");
            return err!(MovieErrors::InvalidRating)
        }
        movie_review.rating = rating;
        movie_review.reviewer = ctx.accounts.user.key();

        let movie_comment_counter = &mut ctx.accounts.movie_comment_counter;
        movie_comment_counter.count = 0;

        let seeds = &["mint".as_bytes(), &[*ctx.bumps.get("reward_mint").unwrap()]];

        let signer = [&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), 
        token::MintTo {
            mint: ctx.accounts.reward_mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.reward_mint.to_account_info()
        }, &signer);

        token::mint_to(cpi_ctx, 10000000)?;
        
        msg!("Movie Review added!");
        Ok(())
    }
    
    pub fn update_movie_review(ctx: Context<UpdateMovieReview>, description: String, rating: u8) -> Result<()> {
        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.description = description;
        movie_review.rating = rating;

        msg!("Movie Review added!");
        Ok(())
    }

    pub fn close_movie_review(_ctx: Context<CloseMovieReview>) -> Result<()> {
        msg!("Movie Review closed!");
        Ok(())
    }

    pub fn create_reward_mint(ctx: Context<CreateRewardToken>, uri: String, name: String, symbol: String) -> Result<()> {
        let seeds = &["mint".as_bytes(), &[*ctx.bumps.get("reward_mint").unwrap()]];

        let signer = [&seeds[..]];

        let account_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.reward_mint.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        invoke_signed(&create_metadata_accounts_v3(ctx.accounts.token_metadata_program.key(),
        ctx.accounts.metadata.key(),
        ctx.accounts.reward_mint.key(),
        ctx.accounts.reward_mint.key(),
        ctx.accounts.user.key(),
        ctx.accounts.user.key(),
        name,
        symbol,
        uri,
        None,
        0,
        true,
        true,
        None,
        None,
        None,
        ), account_info.as_slice(), &signer)?;

        Ok(())
    }

    pub fn add_comment(ctx: Context<AddComment>, comment: String) -> Result<()> {

        let movie_comment_counter = &mut ctx.accounts.movie_comment_counter;
        movie_comment_counter.count += 1;

        let movie_comment = &mut ctx.accounts.comment_pda;

        movie_comment.comment = comment;
        movie_comment.review = ctx.accounts.movie_review.key();
        movie_comment.commentor = ctx.accounts.user.key();
        movie_comment.count = movie_comment.count;

        let seeds = &["mint".as_bytes(), &[*ctx.bumps.get("reward_mint").unwrap()]];

        let signer = [&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), token::MintTo {
            mint: ctx.accounts.reward_mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.reward_mint.to_account_info()
        }, &signer);

        token::mint_to(cpi_ctx, 5000000)?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct AddMovieReview<'info> {
    #[account(init, 
        seeds = [title.as_bytes(), user.key().as_ref() ], 
        bump, 
        payer = user, 
        space = 8 + 32 + 4 + title.len() + 1 + 4 + description.len())
    ]
    pub movie_review: Account<'info, MovieReviewState>,
    #[account(init, seeds = ["counter".as_bytes(), movie_review.key().as_ref()], bump, payer = user, space = 8 + 8)]
    pub movie_comment_counter: Account<'info, MovieCommentCounterState>,
    #[account(mut, seeds = ["mint".as_bytes()], bump)]
    pub reward_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = reward_mint,
        associated_token::authority = user
    )]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
#[instruction(title: String, description:String)]
pub struct UpdateMovieReview<'info> {
    #[account(mut, 
            seeds = [title.as_bytes(), user.key().as_ref()],
            bump,
            realloc = 8 + 32 + 1 + 4 + title.len() + 4 + description. len(),
            realloc::payer = user,
            realloc::zero = true
            )]
    pub movie_review: Account<'info, MovieReviewState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct CloseMovieReview<'info> {
    #[account(mut, close = reviewer, has_one = reviewer)]
    pub movie_review: Account<'info, MovieReviewState>,
    #[account(mut)]
    pub reviewer: Signer<'info>
}

#[derive(Accounts)]
pub struct CreateRewardToken<'info> {
    #[account(init, seeds = ["mint".as_bytes()], bump,  payer = user, mint::decimals = 6, mint::authority = reward_mint)]
    pub reward_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    pub token_metadata_program: AccountInfo<'info>
}

#[derive(Accounts)]
#[instruction(comment: String)]
pub struct AddComment<'info>{
    pub movie_review: Account<'info, MovieReviewState>,
    #[account(
        init, 
        seeds = [movie_review.key().as_ref(), &movie_comment_counter.count.to_le_bytes()],
        bump, 
        payer = user, 
        space = 8 + 32 + 32 + 4 + comment.len() + 8
    )]
    pub comment_pda: Account<'info, MovieComment>,
    #[account(
        mut, 
        seeds = ["counter".as_bytes(), movie_review.key().as_ref()], 
        bump
    )]
    pub movie_comment_counter: Account<'info, MovieCommentCounterState>,
    #[account(
        mut,
        seeds = ["mint".as_bytes().as_ref()],
        bump
    )]
    pub reward_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = reward_mint,
        associated_token::authority = user
    )]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MovieReviewState {
    pub reviewer: Pubkey,
    pub title: String,
    pub rating: u8,
    pub description: String,
}

#[account]
pub struct MovieCommentCounterState{
    pub count: u8
}

#[account]
pub struct MovieComment {
    pub review: Pubkey,
    pub commentor: Pubkey,
    pub comment: String,
    pub count: u64
}

#[error_code]
pub enum MovieErrors {
    #[msg("Ratings need to be between 1 and 5")]
    InvalidRating
}