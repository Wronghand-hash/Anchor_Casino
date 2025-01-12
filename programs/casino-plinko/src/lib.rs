use anchor_lang::prelude::*;

declare_id!("H3NfkuUXGiwNQK8f4xDkqVcPEWxUTRWJ3oLHyAeerGYd");

#[program]
pub mod casino_plinko {
    use super::*;

    /// Initialize the player account
    pub fn initialize_player(ctx: Context<InitializePlayer>, initial_balance: u64) -> Result<()> {
        require!(initial_balance > 0, PlinkoBetError::InvalidInitialBalance);

        let player_account = &mut ctx.accounts.player_account;
        player_account.balance = initial_balance;

        msg!("Player Account Initialized");
        msg!("Player: {}", ctx.accounts.player.key());
        msg!("Initial Balance: {}", initial_balance);

        Ok(())
    }

    /// Initialize the game account
    pub fn initialize_game(ctx: Context<InitializeGame>, initial_balance: u64) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;

        // Initialize balance in game account
        game_account.balance = initial_balance;
        game_account.bet_amount = 0; // No bet yet
        game_account.result = false; // No result yet

        msg!("Game Account Initialized");
        msg!("Initial Game Balance: {}", initial_balance);

        Ok(())
    }

    /// Place a bet
    pub fn place_bet(ctx: Context<PlaceBet>, bet_amount: u64) -> Result<()> {
        let player_account = &mut ctx.accounts.player_account;

        require!(player_account.balance >= bet_amount, PlinkoBetError::InsufficientBalance);

        player_account.balance -= bet_amount;

        let game_account = &mut ctx.accounts.game_account;
        game_account.bet_amount = bet_amount;
        game_account.result = false;

        msg!("Bet placed successfully by {}", ctx.accounts.player.key());
        msg!("Bet Amount: {}", bet_amount);
        msg!("Updated Player Balance: {}", player_account.balance);

        Ok(())
    }

    /// Determine the result of the game
    pub fn determine_result(ctx: Context<DetermineResult>, result: bool) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;
        let player_account = &mut ctx.accounts.player_account;

        game_account.result = result;

        if result {
            let winnings = game_account.bet_amount.checked_mul(2).ok_or(PlinkoBetError::Overflow)?;
            player_account.balance = player_account.balance.checked_add(winnings).ok_or(PlinkoBetError::Overflow)?;
        }

        msg!("Game result determined for player {}", ctx.accounts.player.key());
        msg!("Result: {}", result);
        msg!("Updated Player Balance: {}", player_account.balance);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePlayer<'info> {
    #[account(
        init,
        payer = player,
        space = 8 + 8, // Space for player account (balance)
        seeds = [b"player_account", player.key().as_ref()],
        bump
    )]
    pub player_account: Account<'info, PlayerAccount>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(
        init,
        payer = player,
        space = 8 + 8 + 8 + 1, // Space for game account (balance + bet amount + result)
        seeds = [b"game_account", player.key().as_ref()],
        bump
    )]
    pub game_account: Account<'info, GameAccount>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(
        mut,
        seeds = [b"player_account", player.key().as_ref()],
        bump
    )]
    pub player_account: Account<'info, PlayerAccount>,
    #[account(
        mut,
        seeds = [b"game_account", player.key().as_ref()],
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
        seeds = [b"game_account", player.key().as_ref()],
        bump
    )]
    pub game_account: Account<'info, GameAccount>,
    #[account(
        mut,
        seeds = [b"player_account", player.key().as_ref()],
        bump
    )]
    pub player_account: Account<'info, PlayerAccount>,
    pub player: Signer<'info>,
}

#[account]
pub struct PlayerAccount {
    pub balance: u64,
}

#[account]
pub struct GameAccount {
    pub balance: u64,
    pub bet_amount: u64,
    pub result: bool,
}

#[error_code]
pub enum PlinkoBetError {
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Invalid initial balance")]
    InvalidInitialBalance,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Account already initialized")]
    AlreadyInitialized,
}