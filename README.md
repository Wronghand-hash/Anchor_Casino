# Casino Plinko Smart Contract

This repository contains the smart contract for a decentralized **Plinko** game deployed on the Solana blockchain, written using the **Anchor** framework. The contract allows users to interact with a casino-style Plinko game where players can place bets, determine game results, and receive payouts. Additionally, there is a **Next.js** frontend that interacts with this smart contract, providing a user-friendly interface for players.

## Features
- **Initialize Game**: Set up the casino game account and fund it with SOL.
- **Initialize Player**: Initialize player accounts to track their balance and participation in the game.
- **Place Bet**: Players can place bets by transferring SOL to the game account.
- **Reset Game**: The game can be reset to its initial state by an admin.
- **Determine Result**: The game determines if a player wins or loses based on a multiplier, with the corresponding payout.
- **Top Up Game Account**: Add funds to the game account.
- **Check Balance**: Check the current balance of the game account.

## Table of Contents
- [Installation](#installation)
- [Smart Contract Overview](#smart-contract-overview)
- [Frontend Application](#frontend-application)
- [Usage](#usage)
- [License](#license)

## Installation

### Smart Contract Setup

1. Clone this repository:
    ```bash
    git clone https://github.com/yourusername/casino-plinko.git
    cd casino-plinko
    ```

2. Install dependencies for the Anchor framework:
    ```bash
    npm install -g @project-serum/anchor-cli
    ```

3. Install the required dependencies for the smart contract:
    ```bash
    anchor build
    ```

4. Deploy the contract to the Solana blockchain:
    ```bash
    anchor deploy
    ```

5. After deployment, take note of the **program ID** (which is set in the `declare_id!` macro).

---

### Frontend Setup

The frontend is a **Next.js** application that interacts with the smart contract, providing a UI for users to interact with the Plinko game.

1. Navigate to the frontend directory and install dependencies:
    ```bash
    cd frontend
    npm install
    ```

2. Set up the necessary environment variables:
    - You'll need to create a `.env` file and set up the following variables:
      ```
      NEXT_PUBLIC_SOLANA_NETWORK=devnet # or mainnet-beta
      NEXT_PUBLIC_PROGRAM_ID=your_program_id_here
      ```

3. Run the Next.js frontend:
    ```bash
    npm run dev
    ```

---

## Smart Contract Overview

### GameAccount

The `GameAccount` stores the state of the game, including:
- The current bet amount.
- The result of the game (Pending, Win, or Loss).
- The multiplier for determining the payout.

### PlayerAccount

The `PlayerAccount` stores player-specific information:
- The player's public key.
- The player's current balance in the game.

### Instructions and Functions

- `initialize_game`: Initializes the game account and funds it with SOL.
- `initialize_player`: Initializes a player account with a zero balance.
- `place_bet`: Allows a player to place a bet in SOL.
- `reset_game`: Resets the game account to its initial state.
- `determine_result`: Determines the result of the game and calculates the payout based on a multiplier.
- `top_up_game_account`: Allows the admin to add SOL to the game account.
- `check_balance`: Checks the current balance of the game account.

---

## Frontend Application

The frontend application is built using **Next.js** and allows users to:
- Connect their wallet to interact with the game.
- Place a bet on the Plinko game.
- View their current balance and game results.

### Steps to Interact with the Game

1. **Connect Wallet**: Use Phantom, Sollet, or Solflare wallet to connect to the Solana network.
2. **Place a Bet**: Enter the desired bet amount and place the bet.
3. **Determine Result**: The game result will be displayed, showing whether the player won or lost and the amount of winnings.
4. **Top Up**: Admins can top up the game account with additional funds if needed.
5. **Check Balance**: Players can view the current balance of the game account.

---

## Usage

1. **Initialize the Game**: As an admin, use the `initialize_game` function to set up the game account and fund it with an initial amount of SOL.
2. **Register Players**: Players can register by initializing their player accounts via `initialize_player`.
3. **Place Bets**: Players can use the `place_bet` function to participate in the game by sending SOL to the game account.
4. **Game Result**: The game will determine the result based on a multiplier, and if the player wins, the winnings are transferred to their account.
5. **Reset Game**: If needed, the game can be reset to its initial state using `reset_game`.

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Contributing

We welcome contributions to this project! If you have ideas, improvements, or bug fixes, feel free to open an issue or create a pull request. Make sure to follow the standard GitHub workflow (fork, clone, branch, commit, and pull request).

---

Let me know if you need any further adjustments!
