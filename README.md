# Müe Heroes — Superblock Leaderboard of Legends

Welcome to **Müe Heroes**, an experimental cryptographic leaderboard built to rank wallets that mine rare **superblocks** on the Kaspa network (μ-level ≥ 15). This is a fun proof of concept game for the community to enjoy, this game is designed to reward pariticipation over raw hashrate, making it possible for solo miners and low-power devices to acheieve legendary status. 

---

## ✨ What It Does

🔍 Monitors Kaspa blocks to detect and verify superblocks (μ ≥ 15)  
📊 Assigns points based on the μ-level of each superblock mined  
🏅 Ranks wallets on a dynamic leaderboard based on cumulative score  
🧠 Applies tier-based logic to classify miners into roles (e.g., μScout, μLegend)  
📡 Backend logs and anchors each qualifying win via REST API  
🖥️ Frontend displays real-time leaderboard and seasonal progress  
🎮 Creates a game layer on top of Kaspa mining using rarity mechanics  

---

## 💎 Scoring System μ-Level Points + Tier System

| Tier Name    | μ-Level Range | Points Awarded | Description                                               | 
|--------------|----------------|----------------|----------------------------------------------------------|
| 🧭 μScout     | μ = 15         | 20 pts         | First discoverers of rare terrain.                      |
| 🔨 μForged    | μ = 16–17      | 45 pts         | Hardened miners shaped by the chain.                    |
| 🦁 μLegend    | μ = 18         | 100 pts        | Warriors etched into history.                           |
| 🧙 μMythic    | μ = 19–20      | 250 pts        | Exceedingly rare-nearly mythic.                         |
| 🦍 μHonorius  | μ ≥ 21         | 500+ pts       | “The Honorius Orangutan Elder” Tier — legendary blocks. |

---



**📆 Seasonal Structure**

- **Weekly**: Mini-leaderboard resets to encourage newcomers  
- **Monthly**: Top 10 archived into “_The Book of Müe_”

---

## 👤 Identity System

Players “claim” their wallet by signing a message and registering a **Hero Tag**. This allows leaderboard attribution and seasonal tracking.


---

## 🧠 Kdapp Architecture — How MüE Works

MüE operates in engine managed episodes, where each wallet is treated as its own self-contained game session.  

Events (μ-level blocks) are submitted as commands, and episode logic is applied per wallet.


**Müe Heroes moves away from the traditional model of**:

❌ **Traditional Web App:** 
App → API → Central Database

❌ **Traditional dApp:**  
App → Web3 Wallet → Chain-bound logic execution   

✅ **New Model KdApp:**
Participant → Blockchain → Local Rule Engine → Coordination via HTTP



## Tech Stack Used 

| Layer        | Tech Stack                        | Purpose                                    |
|--------------|-----------------------------------|--------------------------------------------|
| Blockchain   | Kaspa RPC / Devnet                | Source of truth for μ-superblocks          |
| μ-Observer   | Rust (planned)                    | Listens to block headers via RPC           |
| Backend API  | Rust + Axum                       | REST server for score submissions & reads  |
| Frontend UI  | React + Vite + Tailwind CSS       | Dynamic leaderboard + seasonal display     |
| Wallet Auth  | Signature + Hero Tag Claim        | Proves wallet ownership for rankings       |
| Storage      | JSON Lines                        | Score log persistence                      |
| Hosting      | GitHub Pages / IPFS (planned)     | Fully decentralized frontend delivery      |


---

## 🚀 Why This Matters

- **Free to play** dApp leaderboard systems with no smart contracts or fees, just cryptographic proof  
- **Creates a meta-game** around the security of the Kaspa network  
- **Onboards users via fun**, not friction  
- **Celebrates decentralization** make solo mining something to be sought after & fun !


## 🧠 Inspiration

This project was sparked by a claim from a CFA who dismissed Kaspa’s design, arguing that pruning and the fact Kaspa needs 0  archival nodes made it unfit for serious adoption. Müe Heroes was built in direct response to that logic: to demonstrate that **network integrity, fairness, and decentralized participation** can thrive even in a stateless, non-archival architecture.

Instead of debating it, we gamified it 👾.

---

## Coming Soon

- 🦸 Claim your Hero Tag via frontend
- 📜 “Book of Müe” HTML archive hosted on IPFS
- 🔮 +5 pts  Bonus Tier - μOracle Awarded to wallets that *witness* a peer mining a superblock in their DAG-view neighborhood.

---

## 🛠 Dev Quickstart

```bash
git clone https://github.com/mr-fantastic97/Mue-Heroes.git

## Frontend
cd frontend/mue-heroes-ui
npm install
npm run dev

## Backend
cd backend
cargo run


## 🤝 Want to Build ?

Open an issue, fork the repo, or reach out via Twitter @just_code97.  
If you're passionate about Kaspa, Kdapps, or designing decentralized experiences - let's build!