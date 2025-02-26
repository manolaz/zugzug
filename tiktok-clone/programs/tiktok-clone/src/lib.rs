/// Include libraries for program
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token};
use std::mem::size_of;

// Declare program ID
declare_id!("Az4edEtU6JtghfueC4hS7Fo5fG3evPY5VUt6YbNHmhaN");

// Constants
const TEXT_LENGTH: usize = 1024;
const USER_NAME_LENGTH: usize = 100;
const USER_URL_LENGTH: usize = 255;
const VIDEO_URL_LENGTH: usize = 255;
const NUMBER_OF_ALLOWED_LIKES_SPACE: usize = 5;
const NUMBER_OF_ALLOWED_LIKES: u8 = 5;
const CENSORSHIP_THRESHOLD: i64 = -500;

/// TikTok Clone program
#[program]
pub mod tiktok_clone {
    use super::*;

    /// Create state to save the video counts
    /// There is only one state in the program
    /// This account should be initialized before video
    pub fn create_state(ctx: Context<CreateState>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.authority = ctx.accounts.authority.key();
        state.video_count = 0;
        
        emit!(StateCreated {
            authority: state.authority,
        });
        
        Ok(())
    }

    /// Create user
    /// @param name:        user name
    /// @param profile_url: user profile url
    pub fn create_user(
        ctx: Context<CreateUser>,
        name: String,
        profile_url: String
    ) -> Result<()> {
        require!(!name.trim().is_empty(), TiktokError::EmptyUsername);
        require!(!profile_url.trim().is_empty(), TiktokError::EmptyProfileUrl);
        
        let user = &mut ctx.accounts.user;
        user.user_wallet_address = ctx.accounts.authority.key();
        user.user_name = name;
        user.user_profile_image_url = profile_url;
        
        emit!(UserCreated {
            user_wallet: user.user_wallet_address,
            user_name: user.user_name.clone(),
        });
        
        Ok(())
    }

    /// Create video
    pub fn create_video(
        ctx: Context<CreateVideo>,
        description: String,
        video_url: String,
        creator_name: String,
        creator_url: String,
    ) -> Result<()> {
        require!(!description.trim().is_empty(), TiktokError::EmptyDescription);
        require!(!video_url.trim().is_empty(), TiktokError::EmptyVideoUrl);
        
        let state = &mut ctx.accounts.state;
        let video = &mut ctx.accounts.video;
        
        video.authority = ctx.accounts.authority.key();
        video.description = description;
        video.video_url = video_url;
        video.creator_name = creator_name;
        video.creator_url = creator_url;
        video.comment_count = 0;
        video.index = state.video_count;
        video.creator_time = Clock::get()?.unix_timestamp;
        video.likes = 0;
        video.remove = 0;
        video.people_who_liked = Vec::new();

        state.video_count += 1;
        
        emit!(VideoCreated {
            video_id: video.index,
            creator: video.authority,
        });
        
        Ok(())
    }

    /// Create comment for video
    pub fn create_comment(
        ctx: Context<CreateComment>,
        text: String,
        commenter_name: String,
        commenter_url: String,
    ) -> Result<()> {
        let video = &mut ctx.accounts.video;
        
        require!(video.remove > CENSORSHIP_THRESHOLD, TiktokError::VideoRemoved);
        require!(!text.trim().is_empty(), TiktokError::EmptyCommentText);
        
        let comment = &mut ctx.accounts.comment;
        
        comment.authority = ctx.accounts.authority.key();
        comment.text = text;
        comment.commenter_name = commenter_name;
        comment.commenter_url = commenter_url;
        comment.index = video.comment_count;
        comment.video_time = Clock::get()?.unix_timestamp;

        video.comment_count += 1;
        
        emit!(CommentCreated {
            video_id: video.index,
            comment_id: comment.index,
            commenter: comment.authority,
        });
        
        Ok(())
    }

    pub fn approve(ctx: Context<ModerateVideo>) -> Result<()> {
        let video = &mut ctx.accounts.video;
        
        // Ensure only admin can approve
        require!(ctx.accounts.authority.key() == video.authority, TiktokError::UnauthorizedAction);
        
        video.remove += 1;
        
        emit!(VideoModerated {
            video_id: video.index,
            new_status: video.remove,
            is_approved: true,
        });
        
        Ok(())
    }

    pub fn disapprove(ctx: Context<ModerateVideo>) -> Result<()> {
        let video = &mut ctx.accounts.video;
        
        // Ensure only admin can disapprove
        require!(ctx.accounts.authority.key() == video.authority, TiktokError::UnauthorizedAction);
        
        video.remove -= 1;
        
        emit!(VideoModerated {
            video_id: video.index,
            new_status: video.remove,
            is_approved: false,
        });
        
        Ok(())
    }

    pub fn like_video(ctx: Context<LikeVideo>) -> Result<()> {
        let video = &mut ctx.accounts.video;
        let user_key = ctx.accounts.authority.key();

        require!(video.likes < NUMBER_OF_ALLOWED_LIKES, TiktokError::ReachedMaxLikes);
        require!(video.remove > CENSORSHIP_THRESHOLD, TiktokError::VideoRemoved);
        require!(!video.people_who_liked.contains(&user_key), TiktokError::AlreadyLiked);

        video.likes += 1;
        video.people_who_liked.push(user_key);
        
        emit!(VideoLiked {
            video_id: video.index,
            user: user_key,
            total_likes: video.likes,
        });
        
        Ok(())
    }
}

/// Contexts
/// CreateState context
#[derive(Accounts)]
pub struct CreateState<'info> {
    #[account(
        init,
        seeds = [b"state"],
        bump,
        payer = authority,
        space = 8 + size_of::<StateAccount>()
    )]
    pub state: Account<'info, StateAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

/// CreateUser context
#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(
        init,
        seeds = [b"user", authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + size_of::<UserAccount>() + USER_NAME_LENGTH + USER_URL_LENGTH
    )]
    pub user: Account<'info, UserAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

/// CreateVideo context
#[derive(Accounts)]
pub struct CreateVideo<'info> {
    #[account(mut, seeds = [b"state"], bump)]
    pub state: Account<'info, StateAccount>,

    #[account(
        init,
        seeds = [b"video", state.video_count.to_be_bytes().as_ref()],
        bump,
        payer = authority,
        space = 8 + size_of::<VideoAccount>() + TEXT_LENGTH + USER_NAME_LENGTH + USER_URL_LENGTH + VIDEO_URL_LENGTH + 32 * NUMBER_OF_ALLOWED_LIKES_SPACE
    )]
    pub video: Account<'info, VideoAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

/// CreateComment context
#[derive(Accounts)]
pub struct CreateComment<'info> {
    #[account(mut, seeds = [b"video", video.index.to_be_bytes().as_ref()], bump)]
    pub video: Account<'info, VideoAccount>,

    #[account(
        init,
        seeds = [b"comment", video.index.to_be_bytes().as_ref(), video.comment_count.to_be_bytes().as_ref()],
        bump,
        payer = authority,
        space = 8 + size_of::<CommentAccount>() + TEXT_LENGTH + USER_NAME_LENGTH + USER_URL_LENGTH + VIDEO_URL_LENGTH
    )]
    pub comment: Account<'info, CommentAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct LikeVideo<'info> {
    #[account(mut)]
    pub video: Account<'info, VideoAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ModerateVideo<'info> {
    #[account(mut)]
    pub video: Account<'info, VideoAccount>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

// State Account Structure
#[account]
pub struct StateAccount {
    pub authority: Pubkey,
    pub video_count: u64,
}

// User Account Structure
#[account]
pub struct UserAccount {
    pub user_name: String,
    pub user_wallet_address: Pubkey,
    pub user_profile_image_url: String,
}

// Video Account Structure
#[account]
pub struct VideoAccount {
    pub authority: Pubkey,
    pub description: String,
    pub video_url: String,
    pub creator_name: String,
    pub creator_url: String,
    pub comment_count: u64,
    pub index: u64,
    pub creator_time: i64,
    pub people_who_liked: Vec<Pubkey>,
    pub likes: u8,
    pub remove: i64,
}

// Comment Account Structure
#[account]
pub struct CommentAccount {
    pub authority: Pubkey,
    pub text: String,
    pub commenter_name: String,
    pub commenter_url: String,
    pub index: u64,
    pub video_time: i64,
}

// Error enum using Anchor's standard error pattern
#[error_code]
pub enum TiktokError {
    #[msg("Username cannot be empty")]
    EmptyUsername,
    
    #[msg("Profile URL cannot be empty")]
    EmptyProfileUrl,
    
    #[msg("Video description cannot be empty")]
    EmptyDescription,
    
    #[msg("Video URL cannot be empty")]
    EmptyVideoUrl,
    
    #[msg("Comment text cannot be empty")]
    EmptyCommentText,
    
    #[msg("Cannot receive more than 5 likes")]
    ReachedMaxLikes,
    
    #[msg("User has already liked the video")]
    AlreadyLiked,
    
    #[msg("This video has been removed due to community guidelines")]
    VideoRemoved,
    
    #[msg("Only the video owner can perform this action")]
    UnauthorizedAction,
}

// Events for better indexing and tracking
#[event]
pub struct StateCreated {
    pub authority: Pubkey,
}

#[event]
pub struct UserCreated {
    pub user_wallet: Pubkey,
    pub user_name: String,
}

#[event]
pub struct VideoCreated {
    pub video_id: u64,
    pub creator: Pubkey,
}

#[event]
pub struct CommentCreated {
    pub video_id: u64,
    pub comment_id: u64,
    pub commenter: Pubkey,
}

#[event]
pub struct VideoLiked {
    pub video_id: u64,
    pub user: Pubkey,
    pub total_likes: u8,
}

#[event]
pub struct VideoModerated {
    pub video_id: u64,
    pub new_status: i64,
    pub is_approved: bool,
}
