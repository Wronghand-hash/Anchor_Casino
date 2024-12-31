use anchor_lang::prelude::*;

declare_id!("FzPsMc1rLYPkYeWtpkGPrT9kat4886QeFmFbrMtHBAoq");

#[program]
pub mod casino_plinko {
    use super::*;



    // initilize the player account 

    pub fn initilize_player(ctx: Context<InitializePlayer> , initial_balance: u64) -> ProgramResult {
        let player_account= &mut ctx.accounts.player_account;
        player_account.player = *ctx.accouns.player.key;
        player_account.balance = initial_balance;
        Ok(())
    }

    // place bet 

    pub fn place_bet(ctx: Context<PlaceBet> , bet_amount: u64) -> ProgramResult {
        let player_account = &mut ctx.accounts.player_account;
        require!(player_account.balance >= bet_amount, PlinkoBetError::InsufficientBalance);

        player_account.balance -= bet_amount

        let game_account = &mut ctx.accounts.game_account;
        game_account.player = *ctx.accounts.player.key;
        game_account.bet_amount = bet_amount;
        game_account.result = 0; // default to lose
    }


    // determine the result of the game 
    pub fn determine_result(ctx: Context<DetermineResult> , result: u8) -> ProgramResult {
        let game_account = &mut ctx.accounts.game_account;
        let player_account = &mut ctx.accounts.player_account;

        game_account.result = result

        if result == 1 {
            player_account.balance += game_account.bet_amount * 2;
        }

        Ok(())
    }
    

    //context for initilizing player account
    #[derive(Account)]
    pub struct InitializePlayer<'info> {
        #[account(init , payer = player , space= 8+8)]
        pub player_account: Account<'info, PlayerAccount>,
        pub player: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    // context for placing bet

    #[derive(Accounts)]
    pub struct PlaceBet<'info> {
        #[account(mut)]
        pub player_account: Account<'info , PlayerAccount>,
        #[account(
            init , payer = player , space = 8 + 8 + 1
        )]
        pub game_account: Account<'info , GameAccount>,
        pub player: Signer<'info>
    }

    // context for determining the result
    #[derive(Accounts)]
    pub struct DetermineResult<'info> {
        #[account(mut)]
        pub game_account: Account<'info, GameAccount>,
        #[account(mut)]
        pub player_account: Account<'info, PlayerAccount>,
        pub player: Signer<'info>,
}
}


// define the PlayerAccount state
#[account]
pub struct PlayerAccount{ 
    pub player: Pubkey,
    pub balance: u64
}


// define the game account state
#[account]
pub struct GameAccount {
    pub payer: Pubkey,
    pub bet_amount: u64,
    pub result: u8, // 0 for lose and 1 for win
}

// custom errors 

#[error]
pub enum PlinkoBetError {
    #[msg("insufficient balance")]
    InsufficientBalance,
}
