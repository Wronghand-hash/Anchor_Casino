import * as anchor from '@project-serum/anchor';
import { Program, Idl } from '@project-serum/anchor';
import { CasinoPlinko, PlayerAccount, GameAccount } from '../target/types/casino_plinko';
import { expect } from 'chai';
import * as path from 'path';
import * as fs from 'fs';

// Load the Solana wallet keypair
const walletPath = path.join(process.env.HOME || require('os').homedir(), '.config', 'solana', 'id.json');
const walletKeypair = anchor.web3.Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(fs.readFileSync(walletPath, 'utf-8')))
);

// Set the provider URL for Devnet
const provider = new anchor.AnchorProvider(
  new anchor.web3.Connection("https://api.devnet.solana.com"),
  new anchor.Wallet(walletKeypair),
  {}
);
anchor.setProvider(provider);

describe('casino_plinko', () => {
  // Load the program
  const program = anchor.workspace.casino_plinko as Program<CasinoPlinko & Idl>;

  it('Initializes player account and places a bet', async () => {
    // Generate a new keypair for the player account
    const playerAccount = anchor.web3.Keypair.generate();

    // Initialize player account
    await program.methods.initializePlayer(new anchor.BN(100))
      .accounts({
        playerAccount: playerAccount.publicKey,
        player: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([playerAccount])
      .rpc();

    // Fetch the initialized account
    let account = await program.account.playerAccount.fetch(playerAccount.publicKey) as PlayerAccount;
    expect(account.balance.toString()).to.equal('100');

    // Place a bet
    const gameAccount = anchor.web3.Keypair.generate();
    await program.methods.placeBet(new anchor.BN(50))
      .accounts({
        playerAccount: playerAccount.publicKey,
        gameAccount: gameAccount.publicKey,
        player: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([gameAccount])
      .rpc();

    // Fetch the game account
    let game = await program.account.gameAccount.fetch(gameAccount.publicKey) as GameAccount;
    expect(game.betAmount.toString()).to.equal('50');
    expect(game.result).to.equal(0);

    // Determine the result
    await program.methods.determineResult(1)
      .accounts({
        gameAccount: gameAccount.publicKey,
        playerAccount: playerAccount.publicKey,
        player: provider.wallet.publicKey,
      })
      .rpc();

    // Fetch the updated player account
    account = await program.account.playerAccount.fetch(playerAccount.publicKey) as PlayerAccount;
    expect(account.balance.toString()).to.equal('150');
  });
});