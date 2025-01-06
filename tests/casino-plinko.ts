import { describe, it } from 'mocha';  // Correct import
import * as anchor from '@coral-xyz/anchor';
import { Keypair, PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { startAnchor } from 'solana-bankrun';
import type { CasinoPlinko } from '../target/types/casino_plinko';

const IDL = require('../target/idl/casino_plinko.json');

describe('Casino Plinko!', async () => {
    const context = await startAnchor('', [{ name: 'casino_plinko', programId: new PublicKey(IDL.address) }], []);
    const provider = new BankrunProvider(context);

    const payer = provider.wallet as anchor.Wallet;
    const program = new anchor.Program<CasinoPlinko>(IDL, provider);

    const playerAccount = new Keypair();
    let gameAccount: Keypair;

    it('Initialize the player account', async () => {
        const initialBalance = new anchor.BN(1000);

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
        const betAmount = new anchor.BN(100);
        gameAccount = new Keypair();

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
        const result = 1;

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

        const updatedPlayerAccount = await program.account.playerAccount.fetch(playerAccount.publicKey);
        console.log(`Updated Player Balance: ${updatedPlayerAccount.balance}`);
    });
});