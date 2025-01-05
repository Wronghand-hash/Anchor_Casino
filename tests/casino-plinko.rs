use anchor_lang::prelude::*;
use casino_plinko::instructions::*;
use casino_plinko::state::*;
use solana_program_test::*;
use solana_sdk::{signature::Signer, transaction::Transaction};

#[tokio::test]
async fn test_initialize_player() {
    let program_id = casino_plinko::id();
    let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
        "casino_plinko",
        program_id,
        processor!(casino_plinko::processor::Processor::process),
    )
    .start()
    .await;

    let player = Keypair::new();
    let player_account = Keypair::new();

    let mut transaction = Transaction::new_with_payer(
        &[casino_plinko::instruction::initialize_player(
            program_id,
            player_account.pubkey(),
            player.pubkey(),
            100,
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &player_account], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let account_data = banks_client
        .get_account_data(player_account.pubkey())
        .await
        .unwrap()
        .unwrap();
    let player_account_state = PlayerAccount::try_deserialize(&mut &account_data[..]).unwrap();
    assert_eq!(player_account_state.balance, 100);
}