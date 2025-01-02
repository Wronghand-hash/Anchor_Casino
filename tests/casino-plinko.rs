use anchor_lang::prelude::*;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer, system_program, instruction::Instruction};
use std::str::FromStr;

use casino_plinko::ID as PROGRAM_ID;
use casino_plinko::{PlayerAccount, GameAccount};

#[tokio::test]
async fn test_casino_plinko() {
    // Initialize the test environment
    let mut program_test = ProgramTest::new(
        "casino_plinko", // Name of the program
        PROGRAM_ID,      // Program ID
        processor!(casino_plinko::processor::Processor::process_instruction), // Entry point
    );

    // Start the test environment
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create a player account
    let player = Keypair::new();
    let player_pubkey = player.pubkey();

    // Initialize the player account
    let initial_balance = 100;
    let initialize_ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            // Add account metas here
        ],
        data: casino_plinko::instruction::InitializePlayer { initial_balance }.data(),
    };
    let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[initialize_ix],
        Some(&payer.pubkey()),
        &[&payer, &player],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify the player account was initialized correctly
    let player_account_data = banks_client
        .get_account(player_pubkey)
        .await
        .unwrap()
        .unwrap()
        .data;
    let player_account: Account<PlayerAccount> = Account::try_from_unchecked(&player_account_data)?;
    assert_eq!(player_account.player, player_pubkey);
    assert_eq!(player_account.balance, initial_balance);

    // Place a bet
    let bet_amount = 50;
    let game_account = Keypair::new();
    let place_bet_ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            // Add account metas here
        ],
        data: casino_plinko::instruction::PlaceBet { bet_amount }.data(),
    };
    let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[place_bet_ix],
        Some(&payer.pubkey()),
        &[&payer, &player, &game_account],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify the bet was placed correctly
    let player_account_data = banks_client
        .get_account(player_pubkey)
        .await
        .unwrap()
        .unwrap()
        .data;
    let player_account: Account<PlayerAccount> = Account::try_from_unchecked(&player_account_data)?;
    assert_eq!(player_account.balance, initial_balance - bet_amount);

    let game_account_data = banks_client
        .get_account(game_account.pubkey())
        .await
        .unwrap()
        .unwrap()
        .data;
    let game_account: Account<GameAccount> = Account::try_from_unchecked(&game_account_data)?;
    assert_eq!(game_account.player, player_pubkey);
    assert_eq!(game_account.bet_amount, bet_amount);
    assert_eq!(game_account.result, 0);

    // Determine the result of the game (win)
    let result = 1;
    let determine_result_ix = Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            // Add account metas here
        ],
        data: casino_plinko::instruction::DetermineResult { result }.data(),
    };
    let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[determine_result_ix],
        Some(&payer.pubkey()),
        &[&payer, &player],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify the result was determined correctly
    let player_account_data = banks_client
        .get_account(player_pubkey)
        .await
        .unwrap()
        .unwrap()
        .data;
    let player_account: Account<PlayerAccount> = Account::try_from_unchecked(&player_account_data)?;
    assert_eq!(player_account.balance, initial_balance - bet_amount + (bet_amount * 2));

    let game_account_data = banks_client
        .get_account(game_account.pubkey())
        .await
        .unwrap()
        .unwrap()
        .data;
    let game_account: Account<GameAccount> = Account::try_from_unchecked(&game_account_data)?;
    assert_eq!(game_account.result, result);
}