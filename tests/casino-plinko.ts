import { describe, it } from 'node:test';
import * as anchor from '@coral-xyz/anchor';
import { Keypair, PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { startAnchor } from 'solana-bankrun';
import type { CasinoPlinko } from '../target/types/casino_plinko';

const IDL = require('../target/idl/casino_plinko.json');
const PROGRAM_ID = new PublicKey(IDL.address);

describe('Casino Plinko!', async () => {
    const context = await startAnchor('', [{ name: 'casino_plinko', programId: PROGRAM_ID }], []);
    const provider = new BankrunProvider(context);

    const payer = provider.wallet as anchor.Wallet;
    const program = new anchor.Program<CasinoPlinko>(IDL, PROGRAM_ID, provider);

    // Generate a new keypair for the player account
    const playerAccount = new Keypair();
    let gameAccount: Keypair;

    it('Initialize the player account', async () => {
        console.log(`Payer Address      : ${payer.publicKey}`);
        console.log(`Player Account     : ${playerAccount.publicKey}`);

        const initialBalance = new anchor.BN(1000); // Use BN for u64 values

        await program.methods
            .initializePlayer(initialBalance)
            .accounts({
                playerAccount: playerAccount.publicKey,
                player: payer.publicKey,
            })
            .signers([playerAccount])
            .rpc();

        console.log('Player account initialized successfully.');
    });

    it('Place a bet', async () => {
        const betAmount = new anchor.BN(100); // Use BN for u64 values

        // Generate a new keypair for the game account
        gameAccount = new Keypair();

        console.log(`Game Account       : ${gameAccount.publicKey}`);

        await program.methods
            .placeBet(betAmount)
            .accounts({
                playerAccount: playerAccount.publicKey,
                gameAccount: gameAccount.publicKey,
                player: payer.publicKey,
            })
            .signers([gameAccount])
            .rpc();

        console.log('Bet placed successfully.');
    });

    it('Determine the result of the game', async () => {
        const result = 1; // 1 for win, 0 for lose

        // Fetch the game account to get the bet amount
        const gameAccountData = await program.account.gameAccount.fetch(gameAccount.publicKey);
        console.log('Game Account Data:', gameAccountData);

        await program.methods
            .determineResult(result)
            .accounts({
                gameAccount: gameAccount.publicKey,
                playerAccount: playerAccount.publicKey,
                player: payer.publicKey,
            })
            .rpc();

        console.log('Game result determined successfully.');

        // Fetch the updated player account to check the balance
        const updatedPlayerAccount = await program.account.playerAccount.fetch(playerAccount.publicKey);
        console.log(`Updated Player Balance: ${updatedPlayerAccount.balance}`);
    });
});