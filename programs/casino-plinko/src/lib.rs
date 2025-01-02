use anchor_lang::prelude::*;

declare_id!("Gk5Layof7VJwN281YnStuWCfVPWsivkJ8DRJp2brprfv");

#[program]
pub mod casino_plinko {
    use super::*;

    pub fn initialize_player(ctx: Context<InitializePlayer>, initial_balance: u64) -> Result<()> {
        let player_account = &mut ctx.accounts.player_account;
        player_account.player = *ctx.accounts.player.key;
        player_account.balance = initial_balance;
        Ok(())
    }

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

#[derive(Accounts)]
pub struct InitializePlayer<'info> {
    #[account(init, payer = player, space = 8 + 32 + 8)]
    pub player_account: Account<'info, PlayerAccount>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub player_account: Account<'info, PlayerAccount>,
    #[account(init, payer = player, space = 8 + 32 + 8 + 1)]
    pub game_account: Account<'info, GameAccount>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DetermineResult<'info> {
    #[account(mut)]
    pub game_account: Account<'info, GameAccount>,
    #[account(mut)]
    pub player_account: Account<'info, PlayerAccount>,
    pub player: Signer<'info>,
}

#[account]
pub struct PlayerAccount {
    pub player: Pubkey,
    pub balance: u64,
}

#[account]
pub struct GameAccount {
    pub player: Pubkey,
    pub bet_amount: u64,
    pub result: u8, // 0 for lose, 1 for win
}

#[error_code]
pub enum PlinkoBetError {
    #[msg("Insufficient balance")]
    InsufficientBalance,
}