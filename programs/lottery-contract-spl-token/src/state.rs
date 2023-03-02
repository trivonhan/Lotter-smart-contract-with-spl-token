use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct LotteryMaster {
    pub lottery_count: u8,
}

#[account]
#[derive(Default)]
pub struct Lottery {
    pub id: u8,
    pub amount: u64,
    pub is_starting: bool,
    pub player: Vec<Player>,
    pub winner: Player,
    pub claimed: bool,
    pub mint: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug, Copy)]
pub struct Player {
    pub player_account: Pubkey,
    pub player_token_account: Pubkey,
}           