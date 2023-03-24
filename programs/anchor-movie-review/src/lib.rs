use anchor_lang::prelude::*;

declare_id!("G2QagUQXZRktxGae3diNtJhA592W93mBVk5Hy3Usnrqi");

#[program]
pub mod anchor_movie_review {

    use super::*;

    pub fn add_movie_review(ctx: Context<AddMovieReview>, title: String, description: String, rating: u8) -> Result<()> {

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.title = title;
        movie_review.description = description;
        movie_review.rating = rating;
        movie_review.reviewer = ctx.accounts.user.key();
        
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
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
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

#[account]
pub struct MovieReviewState {
    pub reviewer: Pubkey,
    pub title: String,
    pub rating: u8,
    pub description: String,
}
