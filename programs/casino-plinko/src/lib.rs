use anchor_lang::prelude::*;

declare_id!("Byn4gnsR2JgmeyrSXYg4e4iCms2ou56pMV35bEhSWFZk");

#[program]
pub mod casino_plinko {
    use super::*;

    // Initialize the player account
    pub fn initialize_player(ctx: Context<InitializePlayer>, initial_balance: u64) -> Result<()> {
        // Validate initial balance
        require!(initial_balance > 0, PlinkoBetError::InvalidInitialBalance);
    
        let player_account = &mut ctx.accounts.player_account;
        player_account.player = *ctx.accounts.player.key;
        player_account.balance = initial_balance;
    
        // Debugging logs
        msg!("Player account initialized successfully:");
        msg!("Player: {}", ctx.accounts.player.key());
        msg!("Initial Balance: {}", initial_balance);
    
        Ok(())
    }

    // Place a bet
    pub fn place_bet(ctx: Context<PlaceBet>, bet_amount: u64) -> Result<()> {
        let player_account = &mut ctx.accounts.player_account;
        require!(player_account.balance >= bet_amount, PlinkoBetError::InsufficientBalance);

        player_account.balance -= bet_amount;

        let game_account = &mut ctx.accounts.game_account;
        game_account.player = *ctx.accounts.player.key;
        game_account.bet_amount = bet_amount;
        game_account.result = 0; // Default to lose

        Ok(())
    }

    // Determine the result of the game
    pub fn determine_result(ctx: Context<DetermineResult>, result: u8) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;
        let player_account = &mut ctx.accounts.player_account;

        game_account.result = result;

        if result == 1 {
            player_account.balance += game_account.bet_amount * 2;
        }

        Ok(())
    }
}

// Context for initializing the player account
#[derive(Accounts)]
pub struct InitializePlayer<'info> {
    #[account(
        init, // Initialize the account
        payer = player, // The payer is the player
        space = 8 + 32 + 8, // 8 (discriminator) + 32 (player) + 8 (balance)
        seeds = [b"player_account", player.key().as_ref()], // PDA seeds
        bump // Automatically find the bump
    )]
    pub player_account: Account<'info, PlayerAccount>, // The player account
    #[account(mut)]
    pub player: Signer<'info>, // The player signing the transaction
    pub system_program: Program<'info, System>, // The system program
}

// Context for placing a bet
#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub player_account: Account<'info, PlayerAccount>,
    #[account(init, payer = player, space = 8 + 32 + 8 + 1)] // 8 (discriminator) + 32 (player) + 8 (bet_amount) + 1 (result)
    pub game_account: Account<'info, GameAccount>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Context for determining the result
#[derive(Accounts)]
pub struct DetermineResult<'info> {
    #[account(mut)]
    pub game_account: Account<'info, GameAccount>,
    #[account(mut)]
    pub player_account: Account<'info, PlayerAccount>,
    pub player: Signer<'info>,
}

// Define the PlayerAccount state
#[account]
pub struct PlayerAccount {
    pub player: Pubkey, // 32 bytes
    pub balance: u64,   // 8 bytes
}

// Define the GameAccount state
#[account]
pub struct GameAccount {
    pub player: Pubkey, // 32 bytes
    pub bet_amount: u64, // 8 bytes
    pub result: u8,     // 1 byte
}

// Custom errors
#[error_code]
pub enum PlinkoBetError {
    #[msg("Insufficient balance")]
    InsufficientBalance,
}