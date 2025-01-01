import * as anchor from '@project-serum/anchor';
import { Program, Idl } from '@project-serum/anchor';
import { CasinoPlinko } from '../target/types/casino_plinko'; // Import the generated types
import { assert } from 'chai'; // Add this for assertions

// Manually define the PlayerAccount type
type PlayerAccount = {
    player: anchor.web3.PublicKey;
    balance: anchor.BN;
};

describe('plinko-bet', () => {
    const provider = anchor.AnchorProvider.local();
    anchor.setProvider(provider);

    // Load the IDL
    const idl = require('../target/idl/casino_plinko.json');

    // Create the program instance
    const program = new Program<Idl>(idl, idl.metadata.address, provider);

    it('Initializes player account', async () => {
        console.log("Starting test: Initializes player account");

        const playerAccount = anchor.web3.Keypair.generate();
        console.log("Generated player account keypair:", playerAccount.publicKey.toString());

        console.log("Sending initializePlayer transaction...");
        await program.rpc.initializePlayer(new anchor.BN(100), {
            accounts: {
                playerAccount: playerAccount.publicKey,
                player: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [playerAccount],
        });
        console.log("initializePlayer transaction completed.");

        console.log("Fetching player account...");
        const account = await program.account.playerAccount.fetch(playerAccount.publicKey) as PlayerAccount;
        console.log("Player account fetched. Balance:", account.balance.toString());

        assert.ok(account.balance.eq(new anchor.BN(100)), "Player account balance should be 100");
        console.log("Test passed: Player account initialized successfully.");
    });

    it('Places a bet', async () => {
        console.log("Starting test: Places a bet");

        const playerAccount = anchor.web3.Keypair.generate();
        const gameAccount = anchor.web3.Keypair.generate();
        console.log("Generated player account keypair:", playerAccount.publicKey.toString());
        console.log("Generated game account keypair:", gameAccount.publicKey.toString());

        console.log("Sending initializePlayer transaction...");
        await program.rpc.initializePlayer(new anchor.BN(100), {
            accounts: {
                playerAccount: playerAccount.publicKey,
                player: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [playerAccount],
        });
        console.log("initializePlayer transaction completed.");

        console.log("Sending placeBet transaction...");
        await program.rpc.placeBet(new anchor.BN(50), {
            accounts: {
                playerAccount: playerAccount.publicKey,
                gameAccount: gameAccount.publicKey,
                player: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [gameAccount],
        });
        console.log("placeBet transaction completed.");

        console.log("Fetching player account...");
        const account = await program.account.playerAccount.fetch(playerAccount.publicKey) as PlayerAccount;
        console.log("Player account fetched. Balance:", account.balance.toString());

        assert.ok(account.balance.eq(new anchor.BN(50)), "Player account balance should be 50");
        console.log("Test passed: Bet placed successfully.");
    });

    it('Determines result', async () => {
        console.log("Starting test: Determines result");

        const playerAccount = anchor.web3.Keypair.generate();
        const gameAccount = anchor.web3.Keypair.generate();
        console.log("Generated player account keypair:", playerAccount.publicKey.toString());
        console.log("Generated game account keypair:", gameAccount.publicKey.toString());

        console.log("Sending initializePlayer transaction...");
        await program.rpc.initializePlayer(new anchor.BN(100), {
            accounts: {
                playerAccount: playerAccount.publicKey,
                player: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [playerAccount],
        });
        console.log("initializePlayer transaction completed.");

        console.log("Sending placeBet transaction...");
        await program.rpc.placeBet(new anchor.BN(50), {
            accounts: {
                playerAccount: playerAccount.publicKey,
                gameAccount: gameAccount.publicKey,
                player: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [gameAccount],
        });
        console.log("placeBet transaction completed.");

        console.log("Sending determineResult transaction...");
        await program.rpc.determineResult(1, {
            accounts: {
                gameAccount: gameAccount.publicKey,
                playerAccount: playerAccount.publicKey,
                player: provider.wallet.publicKey,
            },
        });
        console.log("determineResult transaction completed.");

        console.log("Fetching player account...");
        const account = await program.account.playerAccount.fetch(playerAccount.publicKey) as PlayerAccount;
        console.log("Player account fetched. Balance:", account.balance.toString());

        assert.ok(account.balance.eq(new anchor.BN(150)), "Player account balance should be 150");
        console.log("Test passed: Result determined successfully.");
    });
});