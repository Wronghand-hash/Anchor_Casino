use anchor_lang::prelude::*;

// Declare the program ID
declare_id!("Byn4gnsR2JgmeyrSXYg4e4iCms2ou56pMV35bEhSWFZk");

#[program]
pub mod casino_plinko {
    use super::*;

    /// Initialize the player account
    pub fn initialize_player(ctx: Context<InitializePlayer>, initial_balance: u64) -> Result<()> {
        // Validate initial balance
        require!(initial_balance > 0, PlinkoBetError::InvalidInitialBalance);

        // Check if the player account already exists
        if ctx.accounts.player_account.to_account_info().data_is_empty() {
            // Set player account data
            let player_account = &mut ctx.accounts.player_account;
            player_account.player = *ctx.accounts.player.key;
            player_account.balance = initial_balance;

            // Debugging logs
            msg!("Initializing Player Account");
            msg!("Player: {}", ctx.accounts.player.key());
            msg!("PDA: {:?}", ctx.accounts.player_account.key());
            msg!("Initial Balance: {}", initial_balance);
        } else {
            // If the account already exists, log a message
            msg!("Player account already exists");
        }

        Ok(())
    }

    /// Place a bet
    pub fn place_bet(ctx: Context<PlaceBet>, bet_amount: u64) -> Result<()> {
        let player_account = &mut ctx.accounts.player_account;

        // Ensure the player has sufficient balance
        require!(
            player_account.balance >= bet_amount,
            PlinkoBetError::InsufficientBalance
        );

        // Deduct bet amount from player balance
        player_account.balance -= bet_amount;

        // Set game account data
        let game_account = &mut ctx.accounts.game_account;
        game_account.player = *ctx.accounts.player.key;
        game_account.bet_amount = bet_amount;
        game_account.result = 0; // Default result

        // Debugging logs
        msg!("Player {} placed a bet of {}", ctx.accounts.player.key(), bet_amount);
        msg!("Updated player balance: {}", player_account.balance);

        Ok(())
    }

    /// Determine the result of the game
    pub fn determine_result(ctx: Context<DetermineResult>, result: u8) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;
        let player_account = &mut ctx.accounts.player_account;

        // Update game result
        game_account.result = result;

        // If the player wins, double the bet amount and update balance
        if result == 1 {
            player_account.balance += game_account.bet_amount * 2;
        }

        // Debugging logs
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
        init_if_needed, // Use `init_if_needed` to avoid errors if the account already exists
        payer = player,
        space = 8 + 32 + 8, // 8 (discriminator) + 32 (player) + 8 (balance)
        seeds = [b"player_account", player.key().as_ref()],
        bump
    )]
    pub player_account: Account<'info, PlayerAccount>, // Player account
    #[account(mut)]
    pub player: Signer<'info>, // Player signing the transaction
    pub system_program: Program<'info, System>, // System program
}

/// Context for placing a bet
#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(
        mut,
        has_one = player, // Ensure the player account belongs to the player
        seeds = [b"player_account", player.key().as_ref()],
        bump
    )]
    pub player_account: Account<'info, PlayerAccount>, // Player account
    #[account(
        init,
        payer = player,
        space = 8 + 32 + 8 + 1, // 8 (discriminator) + 32 (player) + 8 (bet_amount) + 1 (result)
        seeds = [b"game_account", player.key().as_ref()],
        bump
    )]
    pub game_account: Account<'info, GameAccount>, // Game account
    #[account(mut)]
    pub player: Signer<'info>, // Player signing the transaction
    pub system_program: Program<'info, System>, // System program
}

/// Context for determining the result
#[derive(Accounts)]
pub struct DetermineResult<'info> {
    #[account(
        mut,
        has_one = player, // Ensure the game account belongs to the player
        seeds = [b"game_account", player.key().as_ref()],
        bump
    )]
    pub game_account: Account<'info, GameAccount>, // Game account
    #[account(
        mut,
        has_one = player, // Ensure the player account belongs to the player
        seeds = [b"player_account", player.key().as_ref()],
        bump
    )]
    pub player_account: Account<'info, PlayerAccount>, // Player account
    pub player: Signer<'info>, // Player signing the transaction
}

/// Define the PlayerAccount state
#[account]
pub struct PlayerAccount {
    pub player: Pubkey, // Player's public key
    pub balance: u64,   // Player's balance
}

/// Define the GameAccount state
#[account]
pub struct GameAccount {
    pub player: Pubkey,    // Player's public key
    pub bet_amount: u64,   // Bet amount
    pub result: u8,        // Game result
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