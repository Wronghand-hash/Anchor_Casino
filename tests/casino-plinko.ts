import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { CasinoPlinko } from '../target/types/casino_plinko';

describe('casino_plinko', () => {
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.CasinoPlinko as Program<CasinoPlinko>;

  it('Initializes player account', async () => {
    const playerAccount = anchor.web3.Keypair.generate();
    const initialBalance = new anchor.BN(100);

    await program.rpc.initializePlayer(initialBalance, {
      accounts: {
        playerAccount: playerAccount.publicKey,
        player: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [playerAccount],
    });

    const account = await program.account.playerAccount.fetch(playerAccount.publicKey);
    assert.ok(account.player.equals(provider.wallet.publicKey));
    assert.ok(account.balance.eq(initialBalance));
  });

  it('Places a bet', async () => {
    const playerAccount = anchor.web3.Keypair.generate();
    const gameAccount = anchor.web3.Keypair.generate();
    const initialBalance = new anchor.BN(100);
    const betAmount = new anchor.BN(50);

    await program.rpc.initializePlayer(initialBalance, {
      accounts: {
        playerAccount: playerAccount.publicKey,
        player: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [playerAccount],
    });

    await program.rpc.placeBet(betAmount, {
      accounts: {
        playerAccount: playerAccount.publicKey,
        gameAccount: gameAccount.publicKey,
        player: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [gameAccount],
    });

    const account = await program.account.gameAccount.fetch(gameAccount.publicKey);
    assert.ok(account.player.equals(provider.wallet.publicKey));
    assert.ok(account.betAmount.eq(betAmount));
    assert.ok(account.result === 0);
  });

  it('Determines the result of the game', async () => {
    const playerAccount = anchor.web3.Keypair.generate();
    const gameAccount = anchor.web3.Keypair.generate();
    const initialBalance = new anchor.BN(100);
    const betAmount = new anchor.BN(50);

    await program.rpc.initializePlayer(initialBalance, {
      accounts: {
        playerAccount: playerAccount.publicKey,
        player: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [playerAccount],
    });

    await program.rpc.placeBet(betAmount, {
      accounts: {
        playerAccount: playerAccount.publicKey,
        gameAccount: gameAccount.publicKey,
        player: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [gameAccount],
    });

    await program.rpc.determineResult(1, {
      accounts: {
        gameAccount: gameAccount.publicKey,
        playerAccount: playerAccount.publicKey,
        player: provider.wallet.publicKey,
      },
    });

    const gameAccountData = await program.account.gameAccount.fetch(gameAccount.publicKey);
    const playerAccountData = await program.account.playerAccount.fetch(playerAccount.publicKey);

    assert.ok(gameAccountData.result === 1);
    assert.ok(playerAccountData.balance.eq(initialBalance.add(betAmount)));
  });
});