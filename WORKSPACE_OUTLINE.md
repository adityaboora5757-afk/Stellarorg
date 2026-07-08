# Workspace Outline — Stellar dApp Challenge submissions

This workspace is designed to house the sub-projects developed for the Stellar Soroban Challenge program. To keep the project clean, modular, and organized, we will adopt a multi-project monorepo layout.

## Workspace Directory Structure

```text
.
├── PROJECT_CONTEXT.md              # Context, specifications, judging criteria, and instructions
├── WORKSPACE_OUTLINE.md            # Directory structure and outline of sub-projects (this file)
│
├── cadence-flowpoint/              # Payment Streaming Vault (Project Cadence)
│   ├── Cargo.toml                  # Soroban workspace manifest
│   ├── contracts/                  # Soroban Rust contracts
│   │   ├── stream/                 # Core streaming logic & vesting calculation
│   │   └── token/                  # Stellar Asset Contract wrapper or test token
│   ├── frontend/                   # Next.js frontend (static export target)
│   │   ├── next.config.mjs
│   │   └── package.json
│   ├── wrangler.toml               # Cloudflare Workers configuration
│   ├── .github/workflows/          # CI/CD pipeline configuration (Real GitHub Actions)
│   └── README.md                   # Submission README matching Level 3 checklist
│
├── accrue/                         # Stake & Earn Rewards Pool (Project Accrue - Planned)
│   ├── Cargo.toml                  # Soroban workspace manifest
│   ├── contracts/                  # Soroban Rust contracts
│   │   ├── staking/                # Staking custody contract
│   │   ├── rewards/                # Yield accrual calculation contract
│   │   └── token/                  # RWD reward token contract
│   ├── frontend/                   # Next.js frontend (static export target)
│   ├── wrangler.toml               # Cloudflare Workers/Netlify config
│   ├── .github/workflows/          # CI/CD pipeline configuration
│   └── README.md                   # Submission README matching Level 3 checklist
│
└── token-swap-amm/                 # Token Swap Liquidity Pool (AMM - Planned)
    ├── Cargo.toml                  # Soroban workspace manifest
    ├── contracts/                  # Soroban Rust contracts
    │   ├── pool/                   # Constant product pool logic
    │   ├── lp_share/               # LP share token mint/burn logic
    │   └── token/                  # Custom tradeable asset contract
    ├── frontend/                   # Next.js frontend with swap and charts UI
    ├── wrangler.toml               # Cloudflare Workers/Netlify config
    ├── .github/workflows/          # CI/CD pipeline configuration
    └── README.md                   # Submission README matching Level 3 checklist
```

---

## Workspace Setup & Management Guidelines

To build and run these projects locally, ensure the following tools are installed:

1. **Rust & Cargo**: Required for contract development and compilation.
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
2. **Stellar CLI**: Used for compiling contracts, deploying to testnet, and interacting with Soroban RPCs.
   ```bash
   cargo install --locked stellar-cli --features opt
   ```
3. **Node.js (v18+) & npm**: For running the Next.js frontends.
4. **Freighter Wallet**: Configured for Stellar Testnet.

---

## Action Plan per Project

For each sub-project:
1. **Contracts Development (Phase 1)**: Form the contract directories, code the Rust files, write unit tests, and verify them via `cargo test`.
2. **Deployment (Phase 2)**: Deploy compiled `.wasm` contracts to Stellar Testnet and verify on [Stellar Expert](https://stellar.expert/explorer/testnet).
3. **Frontend Integration (Phase 3)**: Set up the Next.js app using StellarWalletsKit, wire it to the deployed contract addresses via environment variables, and build responsive UIs.
4. **CI/CD Pipeline (Phase 4)**: Implement GitHub Actions to test/build contracts and lint/build frontends.
5. **Documentation & Video (Phase 5)**: Create detailed READMEs with screenshots and demo video links.
