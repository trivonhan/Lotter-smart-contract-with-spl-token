use anchor_lang::prelude::*;

use crate::state::{
    LotteryMaster,
    Lottery
};

use crate::external::anchor_spl_token::{
    TokenAccount,
    TokenMint,
  };

use crate::constant::*;

use anchor_spl::*;

#[derive(Accounts)]
pub struct InitializeLotteryMasterContext<'info> {
        /// CHECK: program owner, verified using #access_control
        #[account(mut)]
        pub root: Signer<'info>,
    
        #[account(
            init,
            seeds = [LOTTERY_SEED, root.key().as_ref()],
            bump,
            payer = root,
            space = 8 + 8,
        )]
        pub lottery_master: Box<Account<'info, LotteryMaster>>,
    
        pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct InitLotteryContext<'info> {
    /// CHECK: program owner, verified using #access_control
    #[account(mut)]
    pub root: Signer<'info>,

    #[account(
        mut,
        seeds = [LOTTERY_SEED, root.key().as_ref()],
        bump,
    )]
    pub lottery_master: Box<Account<'info, LotteryMaster>>,

    #[account(
        init,
        seeds = [LOTTERY_SEED, &[lottery_master.lottery_count]],
        bump,
        payer = root,
        space = 1024,
    )]
    pub lottery_state: Box<Account<'info, Lottery>>,

    #[account(
        init,
        seeds = [LOTTERY_ACCOUNT_SEED, &[lottery_master.lottery_count]],
        bump,
        payer = root,
        token::mint = token_mint,
        token::authority = lottery_state,
    )]
    pub lottery_token_account: Box<Account<'info, TokenAccount>>,

    /// CHECK: Token mint like (ERC20)
    #[account(mut)]
    pub token_mint: Account<'info, TokenMint>,

    pub system_program: Program<'info, System>,
    
    /// CHECK: Account program of token
    pub token_program: AccountInfo<'info>
}

#[derive(Accounts)]
#[instruction(_lottery_index: u8)]
pub struct AddMoneyContext<'info> {

    /// CHECK: Who can send money to lottery
    #[account(mut)]
    pub player: Signer<'info>,

    /// CHECK: Associated token account of player
    #[account(mut)]
    pub player_token_account: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [LOTTERY_SEED, &[_lottery_index].as_ref()],
        bump
    )]
    pub lottery_state: Box<Account<'info, Lottery>>,

    /// CHECK: Signer for lottery account
    #[account(
        mut,
        seeds = [LOTTERY_ACCOUNT_SEED, &[_lottery_index]],
        bump,
    )]
    pub lottery_token_account: Box<Account<'info, TokenAccount>>,

    /// CHECK: Token mint like (ERC20)
    #[account(mut)]
    pub token_mint: Account<'info, TokenMint>,

    pub system_program: Program<'info, System>,
    
    /// CHECK: Account program of token
    pub token_program: AccountInfo<'info>
}

#[derive(Accounts)]
#[instruction(_lottery_index: u8)]
pub struct PickWinnerContext<'info> {

    /// CHECK: program owner, verified using #access_control
    #[account(mut)]
    pub root: Signer<'info>,

    #[account(
        mut,
        seeds = [LOTTERY_SEED, &[_lottery_index].as_ref()],
        bump
    )]
    pub lottery_state: Box<Account<'info, Lottery>>,

    pub system_program: Program<'info, System>
}



#[derive(Accounts)]
#[instruction(_lottery_index: u8, _bump: u8)]
pub struct ClaimContext<'info> {

    /// CHECK: Who can send money to lottery
    #[account(mut)]
    pub player: Signer<'info>,

    /// CHECK: Associated token account of player
    #[account(mut)]
    pub player_token_account: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [LOTTERY_SEED, &[_lottery_index].as_ref()],
        bump
    )]
    pub lottery_state: Box<Account<'info, Lottery>>,

    /// CHECK: Signer for lottery account
    #[account(
        mut,
        seeds = [LOTTERY_ACCOUNT_SEED, &[_lottery_index]],
        bump,
    )]
    pub lottery_token_account: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    
    /// CHECK: Account program of token
    pub token_program: AccountInfo<'info>
}