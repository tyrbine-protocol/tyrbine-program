![Solana](https://img.shields.io/badge/Blockchain-Solana-blue.svg)
![License](https://img.shields.io/badge/License-MIT-green.svg)
![GitHub stars](https://img.shields.io/github/stars/tyrbine-protocol/tyrbine-program?style=social)


# Tyrbine
Single-sided liquidity protocol on Solana

Earn trading fees **without impermanent loss** by depositing a single asset into Tyrbine vaults.

---

## Why Tyrbine?

Traditional AMMs require providing liquidity in pairs and expose users to impermanent loss.  
Tyrbine allows you to earn from the growing Solana ecosystem **with a single asset**, making liquidity provision simpler, safer, and more profitable.

---

## Key Features

| Feature | Description |
|---------|-------------|
| **Balancer** | Automatically incentivizes traders to rebalance liquidity by depositing into deficit vaults and withdrawing from surplus vaults, providing the best rates across Solana. |
| **Slippage-Free Swaps** | Powered by Pyth price oracles for accurate and transparent pricing. |
| **Secure Vaults** | Vaults protected by internal reserves and automated circuit breakers to safeguard your funds. |

---

## How It Works

1. Deposit a single asset into a vault.
2. Traders rebalance vaults using the balancer mechanism.
3. Earn trading fees without worrying about impermanent loss.

> Join the existing flow of liquidity on Solana and extract yield from it effortlessly.

---

## Getting Started

1. Go to [tyrbine.com](https://tyrbine.com)
2. Connect your Solana wallet.
3. Select a vault.
4. Deposit your token and start earning fees.

---

## 🛠 Developer Integration

Tyrbine Protocol exposes **4 Solana instructions** that can be called to interact with vaults and manage liquidity.  

### Available Instructions

1. **Staking** – deposit a token into a vault - [How to call it?](https://github.com/tyrbine-protocol/tyrbine-program/blob/main/programs/tyrbine-program/src/instructions/staker/staking.rs#L53)
2. **Unstaking** – withdraw from a vault - [How to call it?](https://github.com/tyrbine-protocol/tyrbine-program/blob/97d127a347140b2804a41a0f732e26a3e49ae907/programs/tyrbine-program/src/instructions/staker/unstaking.rs#L54)
3. **Claim** – claim earned fees - [How to call it?](https://github.com/tyrbine-protocol/tyrbine-program/blob/97d127a347140b2804a41a0f732e26a3e49ae907/programs/tyrbine-program/src/instructions/staker/claim.rs#L43)
4. **Swap** – swap between vaults - [How to call it?](https://github.com/tyrbine-protocol/tyrbine-program/blob/97d127a347140b2804a41a0f732e26a3e49ae907/programs/tyrbine-program/src/instructions/trader/swap.rs#L130)

## Learn More

- [Whitepaper](https://xjx4fw67d3oyxf3nghsgyus2thdtexb26qrqpegmu5cbmxkkf7qa.arweave.net/um_C298e3YuXbTHkbFJamccyXDr0IweQzKdEFl1KL-A)
- [Twitter](https://x.com/tyrbine)
- [Telegram](https://t.me/tyrbine)
