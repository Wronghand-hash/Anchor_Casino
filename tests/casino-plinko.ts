import * as anchor from '@project-serum/anchor';
import { Program, BN } from '@project-serum/anchor';
import { CasinoPlinko } from '../target/types/casino_plinko';
import { expect } from 'chai';

describe('casino_plinko', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.casino_plinko as Program<CasinoPlinko>;

  let playerAccount: anchor.web3.Keypair;
  let gameAccount: anchor.web3.Keypair;
  const initialBalance = new BN(1000); // Use BN for u64 values
  const betAmount = new BN(100); // Use BN for u64 values

  it('Initializes the player account', async () => {
    playerAccount = anchor.web3.Keypair.generate();

    await program.methods.initializePlayer(initialBalance)
      .accounts({
        playerAccount: playerAccount.publicKey,
        player: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([playerAccount])
      .rpc();

    const account = await program.account.playerAccount.fetch(playerAccount.publicKey);
    expect(account.player.toString()).to.equal(provider.wallet.publicKey.toString());
    expect(account.balance.eq(initialBalance)).to.be.true; // Use .eq for BN comparison
  });

  it('Places a bet', async () => {
    gameAccount = anchor.web3.Keypair.generate();

    await program.methods.placeBet(betAmount)
      .accounts({
        playerAccount: playerAccount.publicKey,
        gameAccount: gameAccount.publicKey,
        player: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([gameAccount])
      .rpc();

    const player = await program.account.playerAccount.fetch(playerAccount.publicKey);
    const game = await program.account.gameAccount.fetch(gameAccount.publicKey);

    expect(player.balance.eq(initialBalance.sub(betAmount))).to.be.true; // Use .sub for BN subtraction
    expect(game.player.toString()).to.equal(provider.wallet.publicKey.toString());
    expect(game.betAmount.eq(betAmount)).to.be.true;
    expect(game.result).to.equal(0); // Default to lose
  });

  it('Determines the result of the game (win)', async () => {
    const result = 1; // Win

    await program.methods.determineResult(result)
      .accounts({
        gameAccount: gameAccount.publicKey,
        playerAccount: playerAccount.publicKey,
        player: provider.wallet.publicKey,
      })
      .rpc();

    const player = await program.account.playerAccount.fetch(playerAccount.publicKey);
    const game = await program.account.gameAccount.fetch(gameAccount.publicKey);

    expect(game.result).to.equal(result);
    expect(player.balance.eq(initialBalance.sub(betAmount).add(betAmount.muln(2)))).to.be.true; // Use BN arithmetic
  });

  it('Determines the result of the game (lose)', async () => {
    const result = 0; // Lose

    await program.methods.determineResult(result)
      .accounts({
        gameAccount: gameAccount.publicKey,
        playerAccount: playerAccount.publicKey,
        player: provider.wallet.publicKey,
      })
      .rpc();

    const player = await program.account.playerAccount.fetch(playerAccount.publicKey);
    const game = await program.account.gameAccount.fetch(gameAccount.publicKey);

    expect(game.result).to.equal(result);
    expect(player.balance.eq(initialBalance.sub(betAmount))).to.be.true; // Balance remains the same
  });
});