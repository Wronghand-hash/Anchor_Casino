use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};

// Declare the program ID
declare_id!("EHKxr3iUZD6rvMZ2XaBSYxu76VC9FsYSDZzhKCbdU1jh");

// Constants
const GAME_ACCOUNT_SPACE: usize = 8 + 8 + 1 + 8; // 8 (discriminator) + 8 (bet amount) + 1 (result) + 8 (multiplier)

#[program]
pub mod casino_plinko {
    use super::*;

    /// Initialize the game account and fund it with SOL
    pub fn initialize_game(ctx: Context<InitializeGame>, initial_funding: u64) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;
        let payer = &ctx.accounts.payer;

        // Initialize game account fields
        game_account.bet_amount = 0; // No bet yet
        game_account.result = GameResult::Pending; // No result yet
        game_account.multiplier = 0; // No multiplier yet

        // Transfer SOL from payer's wallet to the game account
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

    /// Place a bet using SOL from the player's wallet
    pub fn place_bet(ctx: Context<PlaceBet>, bet_amount: u64) -> Result<()> {
        require!(bet_amount > 0, PlinkoBetError::InvalidBetAmount);

        let game_account = &mut ctx.accounts.game_account;

        // Ensure the game account is in the correct state
        require!(
            game_account.bet_amount == 0 && game_account.result == GameResult::Pending,
            PlinkoBetError::InvalidGameState
        );

        let player = &ctx.accounts.player;

        // Transfer SOL from player's wallet to the game account
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
        game_account.multiplier = 0; // Reset multiplier

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

        // Ensure the game is in a pending state
        require!(
            game_account.result == GameResult::Pending,
            PlinkoBetError::InvalidGameState
        );

        // Log game account balance before payout
        msg!("Game account balance before payout: {} lamports", game_account.to_account_info().lamports());

        // Determine the result based on the multiplier
        let result = if multiplier > 1 {
            GameResult::Win
        } else {
            GameResult::Loss
        };

        game_account.result = result;
        game_account.multiplier = multiplier;

        if let GameResult::Win = result {
            // Calculate winnings
            let winnings = game_account
                .bet_amount
                .checked_mul(multiplier)
                .ok_or(PlinkoBetError::Overflow)?;

            // Ensure game account has enough lamports
            require!(
                game_account.to_account_info().lamports() >= winnings,
                PlinkoBetError::InsufficientFunds
            );

            // Transfer winnings from game account to player's wallet
            **game_account.to_account_info().try_borrow_mut_lamports()? -= winnings;
            **player.to_account_info().try_borrow_mut_lamports()? += winnings;

            // Log game account balance after payout
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

        // Transfer SOL from payer's wallet to the game account
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
        seeds = [b"global_game_account"], // Shared game account
        bump
    )]
    pub game_account: Account<'info, GameAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(
        mut,
        seeds = [b"global_game_account"], // Shared game account
        bump,
        constraint = game_account.bet_amount == 0 && game_account.result == GameResult::Pending @ PlinkoBetError::InvalidGameState
    )]
    pub game_account: Account<'info, GameAccount>,
    #[account(mut, signer)] // Ensure the player is a signer
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResetGame<'info> {
    #[account(
        mut,
        seeds = [b"global_game_account"], // Shared game account
        bump
    )]
    pub game_account: Account<'info, GameAccount>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DetermineResult<'info> {
    #[account(
        mut,
        seeds = [b"global_game_account"], // Shared game account
        bump
    )]
    pub game_account: Account<'info, GameAccount>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TopUpGameAccount<'info> {
    #[account(
        mut,
        seeds = [b"global_game_account"], // Shared game account
        bump
    )]
    pub game_account: Account<'info, GameAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CheckBalance<'info> {
    #[account(
        mut,
        seeds = [b"global_game_account"], // Shared game account
        bump
    )]
    pub game_account: Account<'info, GameAccount>,
}

#[account]
pub struct GameAccount {
    pub bet_amount: u64,
    pub result: GameResult,
    pub multiplier: u64,
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