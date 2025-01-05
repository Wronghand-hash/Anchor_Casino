import * as anchor from '@project-serum/anchor';
import { Program, Idl } from '@project-serum/anchor';
import { CasinoPlinko } from '../target/types/casino_plinko'; // Import the generated IDL type
import { PublicKey, SystemProgram, Keypair } from '@solana/web3.js';
import assert from 'assert';

describe('casino_plinko', () => {
  // Configure the provider to point to the local Solana cluster
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  // Load the program ID from the declared ID
  const programId = new PublicKey("Byn4gnsR2JgmeyrSXYg4e4iCms2ou56pMV35bEhSWFZk");

  // Create a Program instance using the IDL and program ID
  const program = new Program<CasinoPlinko>(
    require('../target/idl/casino_plinko.json'), // Load the IDL directly
    programId,
    provider
  );

  it('Initializes player account', async () => {
    // Generate a new keypair for the player account
    const playerAccount = Keypair.generate();
    const initialBalance = new anchor.BN(100);

    // Call the initializePlayer instruction
    await program.rpc.initializePlayer(initialBalance, {
      accounts: {
        playerAccount: playerAccount.publicKey,
        player: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [playerAccount],
    });

    // Fetch the player account and verify its data
    const account = await program.account.playerAccount.fetch(playerAccount.publicKey);
    assert.ok(account.player.equals(provider.wallet.publicKey), 'Player public key mismatch');
    assert.ok(account.balance.eq(initialBalance), 'Initial balance mismatch');
  });

  it('Places a bet', async () => {
    // Generate keypairs for the player and game accounts
    const playerAccount = Keypair.generate();
    const gameAccount = Keypair.generate();
    const initialBalance = new anchor.BN(100);
    const betAmount = new anchor.BN(50);

    // Initialize the player account
    await program.rpc.initializePlayer(initialBalance, {
      accounts: {
        playerAccount: playerAccount.publicKey,
        player: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [playerAccount],
    });

    // Call the placeBet instruction
    await program.rpc.placeBet(betAmount, {
      accounts: {
        playerAccount: playerAccount.publicKey,
        gameAccount: gameAccount.publicKey,
        player: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [gameAccount],
    });

    // Fetch the game account and verify its data
    const account = await program.account.gameAccount.fetch(gameAccount.publicKey);
    assert.ok(account.player.equals(provider.wallet.publicKey), 'Player public key mismatch');
    assert.ok(account.betAmount.eq(betAmount), 'Bet amount mismatch');
    assert.strictEqual(account.result, 0, 'Default result should be 0 (lose)');
  });

  it('Determines the result of the game', async () => {
    // Generate keypairs for the player and game accounts
    const playerAccount = Keypair.generate();
    const gameAccount = Keypair.generate();
    const initialBalance = new anchor.BN(100);
    const betAmount = new anchor.BN(50);

    // Initialize the player account
    await program.rpc.initializePlayer(initialBalance, {
      accounts: {
        playerAccount: playerAccount.publicKey,
        player: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [playerAccount],
    });

    // Place a bet
    await program.rpc.placeBet(betAmount, {
      accounts: {
        playerAccount: playerAccount.publicKey,
        gameAccount: gameAccount.publicKey,
        player: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [gameAccount],
    });

    // Call the determineResult instruction with a winning result (1)
    await program.rpc.determineResult(1, {
      accounts: {
        gameAccount: gameAccount.publicKey,
        playerAccount: playerAccount.publicKey,
        player: provider.wallet.publicKey,
      },
    });

    // Fetch the game and player accounts and verify their data
    const gameAccountData = await program.account.gameAccount.fetch(gameAccount.publicKey);
    const playerAccountData = await program.account.playerAccount.fetch(playerAccount.publicKey);

    assert.strictEqual(gameAccountData.result, 1, 'Result should be 1 (win)');
    assert.ok(
      playerAccountData.balance.eq(initialBalance.add(betAmount.mul(new anchor.BN(2)))),
      'Player balance should increase by 2x the bet amount'
    );
  });
});