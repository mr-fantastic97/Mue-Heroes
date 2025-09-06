# Müe Heroes Superblock Leaderboard of Legends

Welcome to **Müe Heroes**, a cryptographic leaderboard built to rank wallets that mine rare **superblocks** on the Kaspa network (μ-level ≥ 15). This is a fun proof of concept game for the community to enjoy, this game is designed to reward pariticipation making it possible for solo miners to acheieve legendary status. 

---

## What It Does

🔍 Monitors Kaspa blocks to detect and verify superblocks (μ ≥ 15)  
📊 Assigns points based on the μ-level of each superblock mined  
🏅 Ranks wallets on a dynamic leaderboard based on cumulative score   
📡 Logs and anchors each qualifying new state
🎮 Aims to Create a game layer on top of Kaspa mining

---

## Scoring System μ-Level Points + Tier System

| μ-Level | Mined Points | Witness Points |
| ------- | ------------ | -------------- |
|    15   |           15 |              7 |
|    16   |           25 |             12 |
|    17   |           40 |             20 |
|    18   |           70 |             35 |
|    19   |          120 |             60 |
|    20   |          200 |            100 |
|   ≥21   |          400 |            200 |

---

## 🧠 KdApp Architecture

Müe Heroes runs per wallet episodes; each wallet has its own game session.
Mine/Witness events are submitted as commands and scored by a local rule engine.

- Local truth: In-memory sessions + JSONL audit log
- Coordination via HTTP, no on-chain execution
- Witness events earn half the mined points
- Demo mode supported; proof verification via Merkle proof will be pluggable later (Doing more research on how to properly implement)

❌ Traditional Web App: App → API → Central DB  
❌ Traditional dApp: App → Wallet → On-chain logic  
✅ KdApp: Participant → (Blockchain or Demo) → Local Rule Engine → HTTP

---

## 🛠 Dev Quickstart (Backend + Frontend)

# 1) Clone
git clone https://github.com/mr-fantastic97/Mue-Heroes.git
cd Mue-Heroes

# 2) Create env files (dev-only example keys replace locally, do not share in prod.)

# backend/.env
mkdir -p backend
cat > backend/.env <<'EOF'
CORS_ORIGINS=http://localhost:5173
MUE_SECRET=dev-submit-key
ADMIN_TOKEN=dev-admin-key
NODE_ENV=development
EOF

# frontend/mue-heroes-react/.env.local
mkdir -p frontend/mue-heroes-react
cat > frontend/mue-heroes-react/.env.local <<'EOF'
VITE_API_URL=http://localhost:8000
VITE_DEV_SUBMIT_KEY=dev-submit-key
VITE_DEV_ADMIN_TOKEN=dev-admin-key
VITE_DEMO_MODE=true     # show Dev Sandbox
# VITE_DEMO_MODE=false  # hide Dev Sandbox
EOF

# 3) Run backend (terminal A)
cd backend
cargo run --bin backend
# -> http://localhost:8000  (leave this running)

# 4) Run frontend (terminal B)
cd ../frontend/mue-heroes-react
npm install
npm run dev
# -> http://localhost:5173

# 5) Have Fun !!!
