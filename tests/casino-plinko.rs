use anchor_lang::prelude::*;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer, system_program};
use std::str::FromStr;

use casino_plinko::ID as PROGRAM_ID; // Replace with your program ID

#[tokio::test]
async fn test_initialize_player_account() {
    println!("Starting test: Initializes player account");

    // Set up the test environment
    let mut program_test = ProgramTest::new("casino_plinko", "93Jyfo2FRNfA78vCwFSi1389rNeSR777wHUyUHZPEk6Xs", processor!(casino_plinko::entry));
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Generate a player account keypair
    let player_account = Keypair::new();
    println!("Generated player account keypair: {}", player_account.pubkey());

    // Create the instruction to initialize the player account
    let instruction = casino_plinko::casino_plinko::initialize_player(
        "93Jyfo2FRNfA78vCwFSi1389rNeSR777wHUyUHZPEk6X",
        player_account.pubkey(),
        payer.pubkey(),
        100,
    );

    // Send the transaction
    let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer, &player_account],
        recent_blockhash,
    );

    println!("Sending initializePlayer transaction...");
    banks_client.process_transaction(transaction).await.unwrap();
    println!("initializePlayer transaction completed.");

    // Fetch the player account
    println!("Fetching player account...");
    let account = banks_client.get_account(player_account.pubkey()).await.unwrap().unwrap();
    let player_account_data = PlayerAccount::try_from_slice(&account.data).unwrap();
    println!("Player account fetched. Balance: {}", player_account_data.balance);

    // Assert the balance is correct
    assert_eq!(player_account_data.balance, 100, "Player account balance should be 100");
    println!("Test passed: Player account initialized successfully.");
}