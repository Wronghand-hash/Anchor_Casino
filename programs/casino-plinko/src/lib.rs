use anchor_lang::prelude::*;

// Declare the program ID
declare_id!("Byn4gnsR2JgmeyrSXYg4e4iCms2ou56pMV35bEhSWFZk");

#[program]
pub mod casino_plinko {
    use super::*;

    /// Initialize the player account
    pub fn initialize_player(ctx: Context<InitializePlayer>, initial_balance: u64) -> Result<()> {
        require!(initial_balance > 0, PlinkoBetError::InvalidInitialBalance);

        let player_account = &mut ctx.accounts.player_account;
        player_account.player = *ctx.accounts.player.key;
        player_account.balance = initial_balance;

        msg!("Initializing Player Account");
        msg!("Player: {}", ctx.accounts.player.key());
        msg!("PDA: {:?}", ctx.accounts.player_account.key());
        msg!("Initial Balance: {}", initial_balance);

        Ok(())
    }

    /// Place a bet
    pub fn place_bet(ctx: Context<PlaceBet>, bet_amount: u64) -> Result<()> {
        let player_account = &mut ctx.accounts.player_account;

        require!(
            player_account.balance >= bet_amount,
            PlinkoBetError::InsufficientBalance
        );

        player_account.balance -= bet_amount;

        let game_account = &mut ctx.accounts.game_account;
        game_account.player = *ctx.accounts.player.key;
        game_account.bet_amount = bet_amount;
        game_account.result = 0;

        msg!("Player {} placed a bet of {}", ctx.accounts.player.key(), bet_amount);
        msg!("Updated player balance: {}", player_account.balance);

        Ok(())
    }

    /// Determine the result of the game
    pub fn determine_result(ctx: Context<DetermineResult>, result: u8) -> Result<()> {
        require!(result <= 1, PlinkoBetError::Unauthorized);

        let game_account = &mut ctx.accounts.game_account;
        let player_account = &mut ctx.accounts.player_account;

        game_account.result = result;

        if result == 1 {
            player_account.balance = player_account.balance
                .checked_add(game_account.bet_amount.checked_mul(2).unwrap())
                .unwrap();
        }

        msg!("Game result determined for player {}", ctx.accounts.player.key());
        msg!("Result: {}", result);
        msg!("Updated player balance: {}", player_account.balance);

        Ok(())
    }
}

/// Context for initializing the player account
#[derive(Accounts)]
pub struct InitializePlayer<'info> {
    #[account(
        init_if_needed,
        payer = player,
        space = 8 + 32 + 8,
        seeds = [b"player_account", player.key().as_ref()],
        bump
    )]
    pub player_account: Account<'info, PlayerAccount>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Context for placing a bet
#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(
        mut,
        has_one = player,
        seeds = [b"player_account", player.key().as_ref()],
        bump
    )]
    pub player_account: Account<'info, PlayerAccount>,
    #[account(
        init,
        payer = player,
        space = 8 + 32 + 8 + 1,
        seeds = [b"game_account", player.key().as_ref()],
        bump
    )]
    pub game_account: Account<'info, GameAccount>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Context for determining the result
#[derive(Accounts)]
pub struct DetermineResult<'info> {
    #[account(
        mut,
        has_one = player,
        seeds = [b"game_account", player.key().as_ref()],
        bump
    )]
    pub game_account: Account<'info, GameAccount>,
    #[account(
        mut,
        has_one = player,
        seeds = [b"player_account", player.key().as_ref()],
        bump
    )]
    pub player_account: Account<'info, PlayerAccount>,
    pub player: Signer<'info>,
}

/// Define the PlayerAccount state
#[account]
pub struct PlayerAccount {
    pub player: Pubkey,
    pub balance: u64,
}

/// Define the GameAccount state
#[account]
pub struct GameAccount {
    pub player: Pubkey,
    pub bet_amount: u64,
    pub result: u8,
}

/// Custom errors
#[error_code]
pub enum PlinkoBetError {
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Invalid initial balance")]
    InvalidInitialBalance,
    #[msg("Unauthorized access")]
    Unauthorized,
}