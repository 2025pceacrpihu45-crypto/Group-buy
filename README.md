# 🛒 Bulk Order Pooling Smart Contract (Soroban)

## 📌 Project Description
This project is a Soroban smart contract on the Stellar blockchain that allows users to pool their funds together to reach a bulk purchase threshold.

The goal is to enable collaborative buying, where users can unlock discounts that are normally only available for large orders.

---

## ⚙️ What It Does
- Users contribute funds to a shared pool
- The contract tracks total contributions
- Each user's contribution is stored
- A threshold defines when the bulk discount is unlocked
- Once the threshold is reached, the pool is marked as completed

---

## ✨ Features
- 👥 Multi-user participation
- 🎯 Threshold-based completion
- 🔐 Secure user authentication
- 📊 Individual contribution tracking
- ⚡ Built using Soroban (fast & low cost)

---

## 📂 Functions

### `init(threshold)`
Initializes the pool with a target amount.

### `contribute(user, amount)`
Allows users to contribute funds to the pool.

### `get_pool()`
Returns pool status (total amount, threshold, completed or not).

### `get_contribution(user)`
Returns how much a user has contributed.

---

## 🧱 Tech Stack
- Rust
- Soroban SDK
- Stellar Blockchain

---

## 🚀 Future Scope
- Add deadline for pool
- Refund if threshold not reached
- Token integration (real payments)
- Event logging for frontend
- Marketplace integration

---

## ⚠️ Disclaimer
This is a basic prototype and not production-ready.

---

## 📄 License
MIT License
Wallet address: GDWJXFYV26LYM6QFWQHD44QUTPAP4Y5LF6GJTRZKJMVSJT5OZALRMASH

contract address: CBDJ7KKA32Z2JRRQ3DNAESTY7MRS2QQ5TMNMM3KOK5RW5J2CJVYZ24S5

https://stellar.expert/explorer/testnet/contract/CBDJ7KKA32Z2JRRQ3DNAESTY7MRS2QQ5TMNMM3KOK5RW5J2CJVYZ24S5

<img width="1908" height="1077" alt="image" src="https://github.com/user-attachments/assets/bd4d6168-543d-48c2-a76c-73a0b4305fbb" />
