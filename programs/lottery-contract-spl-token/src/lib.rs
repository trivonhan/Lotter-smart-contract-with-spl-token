use anchor_lang::prelude::*;
pub mod constant;
pub mod error;
pub mod external;
pub mod context;
pub mod state;

use crate::external::spl_token::{
    TokenAccount,
  };

use crate::context::*;

use anchor_lang::solana_program::{
    instruction:: {
        Instruction,
    },
    system_instruction::{
        transfer
    },
    program::{
        invoke, invoke_signed
    }
};

use crate::external::spl_token::{
    ID as TOKEN_PROGRAM_ID,
  };

use crate::{
    constant::*,
    error::{
        ErrorCode
    }
};

declare_id!("2eXdd4PCc8eLbgTv5WVhu5exQbXcEzCvybbW2181NnPN");

#[program]
pub mod lottery_contract_spl_token {
    use crate::state::Player;

    use super::*;

    #[access_control(is_root(*ctx.accounts.root.key))]
    pub fn init_lottery_master(ctx: Context<InitializeLotteryMasterContext>) -> Result<()> {
        let lottery_master = &mut ctx.accounts.lottery_master;

        lottery_master.lottery_count = 0;

        Ok(())
    }

    #[access_control(is_root(*ctx.accounts.root.key))]
    pub fn init_lottery(ctx: Context<InitLotteryContext>) -> Result<()>{
        msg!("Initialize lottery");
        let lottery_state = &mut ctx.accounts.lottery_state;
        let lottery_master = &mut ctx.accounts.lottery_master;
        let lottery_associated_account = &mut ctx.accounts.lottery_token_account;

        lottery_state.amount = 0;
        lottery_state.is_starting = true;
        lottery_state.player = vec![];
        lottery_state.claimed = false;
        lottery_state.id = lottery_master.lottery_count;
        lottery_state.mint = lottery_associated_account.mint;

        lottery_master.lottery_count += 1;

        Ok(())
    }

    pub fn add_money_to_lottery(ctx: Context<AddMoneyContext>, _lottery_index: u8) -> Result<()> {
        let player = &ctx.accounts.player;
        let lottery_state = &mut ctx.accounts.lottery_state;
        let lottery_token_account =  &mut ctx.accounts.lottery_token_account;
        let player_token_account = &mut ctx.accounts.player_token_account;
        let player_lottery = Player {
          player_account: player.key(),
          player_token_account: player_token_account.key(),
        };

        require!(lottery_state.is_starting==true, ErrorCode::LotteryNotStart);
        
        transfer_token(&player, &player_token_account, &lottery_token_account.to_account_info(), 1000000000, &[])
            .expect("transfer fail");

        lottery_state.amount += 1000000000;
        lottery_state.player.push(player_lottery);

        Ok(())
    }

    #[access_control(is_root(*ctx.accounts.root.key))]
    pub fn pick_winner(ctx: Context<PickWinnerContext>, _lottery_index: u8) -> Result<()> {
        let lottery_state = &mut ctx.accounts.lottery_state;

        let amount_player = lottery_state.player.len();
        let player = &lottery_state.player;
        let now_ts = Clock::get().unwrap().unix_timestamp;

        msg!("Time is: {:?}", now_ts);
        let winner_index = now_ts % amount_player as i64;
        msg!("Winner is: {:?}", player[winner_index as usize]);


        lottery_state.is_starting = false;
        lottery_state.winner = lottery_state.player[winner_index as usize];

        msg!("lottery account winner: {:?}", lottery_state.winner.player_account);

        Ok(())
    }

    pub fn claim(ctx: Context<ClaimContext>, _lottery_index: u8, _bump: u8) -> Result<()> {
      let lottery_state = &mut ctx.accounts.lottery_state;
      let lottery_token_account = &mut ctx.accounts.lottery_token_account;
      let player_token_account = &mut ctx.accounts.player_token_account;

      let player  = &ctx.accounts.player;

      require!(lottery_state.winner.player_account == player.key(), ErrorCode::NotTheWinner);

      msg!("Player: {:?}", player.key());

      let lottery_id = lottery_state.id;

      let seed: &[&[u8]] = &[
            LOTTERY_SEED, 
            &[_lottery_index],
            &[_bump]
          ];
      
      transfer_token(
          &lottery_state.to_account_info(), 
          &lottery_token_account.to_account_info(), 
          &player_token_account, 
          lottery_state.amount, 
          &[&seed]
        )
        .expect("transfer fail");

      lottery_state.amount = 0;
      lottery_state.claimed = true;

      Ok(())
  }

}

pub fn is_root(user: Pubkey) -> Result<()> {
    let user_key = user.to_string();
    let result = ROOT_KEYS.iter().position(|&key| key == &user_key[..]);
    if result == None {
      return Err(ErrorCode::Unauthorized.into());
    }
  
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Default)]
pub struct TransferTokenParams {
  pub instruction: u8,
  pub amount: u64,
}

pub fn transfer_token<'a>(
  owner: &AccountInfo<'a>,
  from_pubkey: &AccountInfo<'a>,
  to_pubkey: &AccountInfo<'a>,
  amount: u64,
  signer_seeds: &[&[&[u8]]],
) -> std::result::Result<(), ProgramError> {
  let data = TransferTokenParams {
    instruction: 3,
    amount,
  };
  let instruction = Instruction {
    program_id: TOKEN_PROGRAM_ID,
    accounts: vec![
      AccountMeta::new(*from_pubkey.key, false),
      AccountMeta::new(*to_pubkey.key, false),
      AccountMeta::new_readonly(*owner.key, true),
    ],
    data: data.try_to_vec().unwrap(),
  };
  if signer_seeds.len() == 0 {
    invoke(&instruction, &[from_pubkey.clone(), to_pubkey.clone(), owner.clone()])
  }
  else {
    invoke_signed(&instruction, &[from_pubkey.clone(), to_pubkey.clone(), owner.clone()], &signer_seeds)
  }
}