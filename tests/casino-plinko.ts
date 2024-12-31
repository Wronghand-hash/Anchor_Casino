import * as anchor from '@project-serum/anchor';
import {Program} from '@project-serum/anchor';
import {CasinoPlinko} from '../target/types/casino_plinko';

describe('plinko-bet', () => {
    const provider = anchor.Provider.local();
    anchor.setProvider(provider);

    const program = anchor.workspace.PlinkoBet as Program<CasinoPlinko>;

    it('Initializes player account', async () => {
        const playerAccount = anchor.web3.Keypair.generate();
        await program.rpc.initializePlayer(new anchor.BN(100), {
            accounts: {
                playerAccount: playerAccount.publicKey,
                player: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [playerAccount],
        });

        let account = await program.account.playerAccount.fetch(playerAccount.publicKey);
        assert.ok(account.balance.eq(new anchor.BN(100)));
    });

    it('Places a bet', async () => {
        const playerAccount = anchor.web3.Keypair.generate();
        const gameAccount = anchor.web3.Keypair.generate();

        await program.rpc.initializePlayer(new anchor.BN(100), {
            accounts: {
                playerAccount: playerAccount.publicKey,
                player: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [playerAccount],
        });

        await program.rpc.placeBet(new anchor.BN(50), {
            accounts: {
                playerAccount: playerAccount.publicKey,
                gameAccount: gameAccount.publicKey,
                player: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [gameAccount],
        });

        let account = await program.account.playerAccount.fetch(playerAccount.publicKey);
        assert.ok(account.balance.eq(new anchor.BN(50)));
    });

    it('Determines result', async () => {
        const playerAccount = anchor.web3.Keypair.generate();
        const gameAccount = anchor.web3.Keypair.generate();

        await program.rpc.initializePlayer(new anchor.BN(100), {
            accounts: {
                playerAccount: playerAccount.publicKey,
                player: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [playerAccount],
        });

        await program.rpc.placeBet(new anchor.BN(50), {
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

        let account = await program.account.playerAccount.fetch(playerAccount.publicKey);
        assert.ok(account.balance.eq(new anchor.BN(150)));
    });
});