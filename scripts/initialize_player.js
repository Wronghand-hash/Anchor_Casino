const anchor = require("@project-serum/anchor");
const { PublicKey, SystemProgram } = anchor.web3;

// Load the IDL
const idl = require("../target/idl/plinko_bet.json");

async function initializePlayer() {
    console.log("Setting up provider...");
    const provider = anchor.Provider.local();
    anchor.setProvider(provider);
    console.log("Provider set up successfully.");

    console.log("Loading program...");
    const programId = new PublicKey("2JU147Qh1s54RJZPFdyZPvhNDJMpUfaHEHE4UPG27M7m");
    const program = new anchor.Program(idl, programId, provider);
    console.log("Program loaded successfully.");

    console.log("Generating player account keypair...");
    const playerAccount = anchor.web3.Keypair.generate();
    console.log("Player account keypair generated:", playerAccount.publicKey.toString());

    console.log("Initializing player account...");
    try {
        await program.rpc.initializePlayer(new anchor.BN(100), {
            accounts: {
                playerAccount: playerAccount.publicKey,
                player: provider.wallet.publicKey,
                systemProgram: SystemProgram.programId,
            },
            signers: [playerAccount],
        });
        console.log("Player account initialized successfully.");
    } catch (error) {
        console.error("Error initializing player account:", error);
    }

    console.log("Player account initialized:", playerAccount.publicKey.toString());
}

console.log("Starting initializePlayer script...");
initializePlayer()
    .then(() => console.log("Script execution completed."))
    .catch((err) => console.error("Script execution failed:", err));