use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use std::mem::size_of;

declare_id!("2nA5CFiicnJb33pQQkJ5GGP2166CySwXSWFgRgRsG1DF");

const GAME_ACCOUNT_SPACE: usize = 8 + 8 + 1 + 8;
const PLAYER_ACCOUNT_SPACE: usize = 8 + 32 + 8;

#[program]
pub mod casino_plinko {
    use super::*;

    /// Initialize the game account and fund it with SOL
    pub fn initialize_game(ctx: Context<InitializeGame>, initial_funding: u64) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;
        let payer = &ctx.accounts.payer;

        game_account.bet_amount = 0;
        game_account.result = GameResult::Pending;
        game_account.multiplier = 0;

        let transfer_instruction = Transfer {
            from: payer.to_account_info(),
            to: game_account.to_account_info(),
        };
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_instruction,
        );
        transfer(cpi_context, initial_funding)?;

        emit!(GameInitialized {
            game: ctx.accounts.game_account.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        msg!("Game Account Initialized");
        msg!("Game: {}", ctx.accounts.game_account.key());
        msg!("Initial Funding: {} lamports", initial_funding);

        Ok(())
    }

    /// Initialize a player account
    pub fn initialize_player(ctx: Context<InitializePlayer>) -> Result<()> {
        let player_account = &mut ctx.accounts.player_account;
        player_account.player = *ctx.accounts.player.key;
        player_account.balance = 0;

        emit!(PlayerInitialized {
            player: ctx.accounts.player.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        msg!("Player Account Initialized");
        msg!("Player: {}", ctx.accounts.player.key());

        Ok(())
    }

    /// Place a bet using SOL from the player's wallet
    pub fn place_bet(ctx: Context<PlaceBet>, bet_amount: u64) -> Result<()> {
        require!(bet_amount > 0, PlinkoBetError::InvalidBetAmount);

        let game_account = &mut ctx.accounts.game_account;

        require!(
            game_account.bet_amount == 0 && game_account.result == GameResult::Pending,
            PlinkoBetError::InvalidGameState
        );

        let player = &ctx.accounts.player;

        let transfer_instruction = Transfer {
            from: player.to_account_info(),
            to: game_account.to_account_info(),
        };
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_instruction,
        );
        transfer(cpi_context, bet_amount)?;

        game_account.bet_amount = bet_amount;
        game_account.result = GameResult::Pending;
        game_account.multiplier = 0;

        emit!(BetPlaced {
            player: player.key(),
            bet_amount,
            timestamp: Clock::get()?.unix_timestamp,
        });

        msg!("Bet placed successfully by {}", player.key());
        msg!("Bet Amount: {}", bet_amount);

        Ok(())
    }

    /// Reset the game account to its initial state
    pub fn reset_game(ctx: Context<ResetGame>) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;
        game_account.bet_amount = 0;
        game_account.result = GameResult::Pending;
        game_account.multiplier = 0;

        emit!(GameReset {
            game: ctx.accounts.game_account.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        msg!("Game Account Reset");
        msg!("Game: {}", ctx.accounts.game_account.key());

        Ok(())
    }

    /// Determine the result of the game and transfer winnings to the player
    pub fn determine_result(ctx: Context<DetermineResult>, multiplier: u64) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;
        let player = &ctx.accounts.player;

        require!(
            game_account.result == GameResult::Pending,
            PlinkoBetError::InvalidGameState
        );

        msg!("Game account balance before payout: {} lamports", game_account.to_account_info().lamports());

        let result = if multiplier > 1 {
            GameResult::Win
        } else {
            GameResult::Loss
        };

        game_account.result = result;
        game_account.multiplier = multiplier;

        if let GameResult::Win = result {
            let winnings = game_account
                .bet_amount
                .checked_mul(multiplier)
                .ok_or(PlinkoBetError::Overflow)?;

            require!(
                game_account.to_account_info().lamports() >= winnings,
                PlinkoBetError::InsufficientFunds
            );

            **game_account.to_account_info().try_borrow_mut_lamports()? -= winnings;
            **player.to_account_info().try_borrow_mut_lamports()? += winnings;

            msg!("Game account balance after payout: {} lamports", game_account.to_account_info().lamports());
            msg!("Winnings transferred: {} lamports", winnings);
        }

        emit!(ResultDetermined {
            player: player.key(),
            result: game_account.result,
            winnings: if let GameResult::Win = result {
                game_account.bet_amount * multiplier
            } else {
                0
            },
            timestamp: Clock::get()?.unix_timestamp,
        });

        msg!("Game result determined for player {}", player.key());
        msg!("Result: {:?}", game_account.result);

        Ok(())
    }

    /// Top up the game account with additional funds
    pub fn top_up_game_account(ctx: Context<TopUpGameAccount>, amount: u64) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;
        let payer = &ctx.accounts.payer;

        let transfer_instruction = Transfer {
            from: payer.to_account_info(),
            to: game_account.to_account_info(),
        };
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_instruction,
        );
        transfer(cpi_context, amount)?;

        msg!("Game account topped up with {} lamports", amount);
        Ok(())
    }

    /// Check the balance of the game account
    pub fn check_balance(ctx: Context<CheckBalance>) -> Result<()> {
        let game_account = &ctx.accounts.game_account;
        msg!("Game account balance: {} lamports", game_account.to_account_info().lamports());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(
        init,
        payer = payer,
        space = GAME_ACCOUNT_SPACE,
        seeds = [b"global_game_account"],
        bump
    )]
    pub game_account: Account<'info, GameAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializePlayer<'info> {
    #[account(
        init,
        seeds = [b"game_account", player.key().as_ref()],
        bump,
        payer = player,
        space = PLAYER_ACCOUNT_SPACE,
    )]
    pub player_account: Account<'info, PlayerAccount>,
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut, seeds = [b"global_game_account"], bump)]
    pub game_account: Account<'info, GameAccount>,
    #[account(mut, signer)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResetGame<'info> {
    #[account(mut, seeds = [b"global_game_account"], bump)]
    pub game_account: Account<'info, GameAccount>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DetermineResult<'info> {
    #[account(mut, seeds = [b"global_game_account"], bump)]
    pub game_account: Account<'info, GameAccount>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TopUpGameAccount<'info> {
    #[account(mut, seeds = [b"global_game_account"], bump)]
    pub game_account: Account<'info, GameAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CheckBalance<'info> {
    #[account(mut, seeds = [b"global_game_account"], bump)]
    pub game_account: Account<'info, GameAccount>,
}

#[account]
pub struct GameAccount {
    pub bet_amount: u64,
    pub result: GameResult,
    pub multiplier: u64,
}

#[account]
pub struct PlayerAccount {
    pub player: Pubkey,
    pub balance: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum GameResult {
    Pending,
    Win,
    Loss,
}

#[event]
pub struct GameInitialized {
    pub game: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct PlayerInitialized {
    pub player: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct BetPlaced {
    pub player: Pubkey,
    pub bet_amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct GameReset {
    pub game: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ResultDetermined {
    pub player: Pubkey,
    pub result: GameResult,
    pub winnings: u64,
    pub timestamp: i64,
}

#[error_code]
pub enum PlinkoBetError {
    #[msg("Invalid bet amount")]
    InvalidBetAmount,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Invalid game state")]
    InvalidGameState,
    #[msg("Insufficient funds in game account")]
    InsufficientFunds,
}
