# Plan: Align README Project Structure with Actual Repository (Issue #230)

## Context

The README has three problems a new contributor hits immediately:
1. The **Project Structure tree** is incomplete — it omits `docs/`, `fuzz/`, key root-level files, and the full `scripts/` listing.
2. The **Getting Started section** (lines 95–130) has broken markdown (unclosed code fence, `bashcd` instead of proper shell blocks), uses the verbose `cargo build --release --target wasm32-unknown-unknown` instead of the preferred `stellar contract build`, and references non-existent `agent/` and `frontend/` directories with malformed "Run the AI Agent" / "Run the Frontend" instructions.
3. No cross-links from Getting Started to `ARCHITECTURE.md`, `CONTRIBUTING.md`, or `scripts/README-E2E.md`.

---

## 1. Summary

Only `README.md` needs to be edited. No new files. No code changes.

**Changes at a glance:**
- Replace the Project Structure tree with an accurate one
- Fix and clean up the Getting Started section (broken fences, build command, non-existent paths)
- Remove the broken "Run the AI Agent" and "Run the Frontend" blocks; replace with a clearly-labelled **Planned Components** section
- Add cross-links to `ARCHITECTURE.md`, `CONTRIBUTING.md`, and `scripts/README-E2E.md`

---

## 2. Files to Modify

| File | Why |
|------|-----|
| `README.md` | The only file that needs to change — project tree, Getting Started, planned components, cross-links |

No other files are affected.

---

## 3. Implementation Steps (in order)

### Step 1 — Replace the Project Structure tree (lines 72–93)

Replace the current tree with one that reflects the real layout:

```
NeuroWealth-Smartcontract/
├── neurowealth-vault/          # Soroban smart contracts workspace
│   ├── Cargo.toml
│   ├── contracts/
│   │   └── vault/              # Core vault contract
│   │       ├── Cargo.toml
│   │       └── src/
│   │           ├── lib.rs      # Contract logic, events, error types
│   │           └── topics.rs   # Exported event topic constants
│   └── fuzz/                   # Libfuzzer fuzz targets
├── scripts/                    # Deployment and utility scripts
│   ├── deploy-devnet.sh        # One-command devnet deploy
│   ├── e2e-devnet.sh           # End-to-end devnet tests
│   ├── verify-deployment.sh
│   ├── generate-spec.py        # Generate contract-spec.json
│   ├── validate-spec.py
│   ├── README-E2E.md           # E2E test guide
│   └── README-SPEC.md          # Spec generation guide
├── docs/
│   ├── MAINNET_CHECKLIST.md    # Pre-mainnet sign-off checklist
│   ├── UPGRADE_MIGRATION.md    # Contract upgrade guide
│   └── WASM_SIZE.md            # WASM size tracking
├── .env.devnet.template        # Environment variable template
├── deny.toml                   # cargo-deny dependency audit config
├── ARCHITECTURE.md             # Storage layout, data flows, invariants
├── EVENTS.md                   # Full event schema reference
├── SECURITY.md                 # Trust model and threat analysis
├── CONTRIBUTING.md             # Development setup and PR process
├── CHANGELOG.md
└── README.md
```

Below the tree, add a **Planned Components** subsection:

```markdown
### Planned Components

The following are not yet in this repository. Each will be added as a
separate directory once development begins:

| Component | Directory | Status |
|-----------|-----------|--------|
| AI agent backend (Node.js/Python) | `agent/` | Planned |
| Next.js web frontend | `frontend/` | Planned |
| WhatsApp bot handler | `whatsapp/` | Planned |
```

> **Note:** Issue #230 asks for "links to tracking issues" for planned components. If specific GitHub issue numbers exist for agent/frontend/whatsapp work, they should be added to the table above as links. If they don't exist yet, the table alone satisfies the "clearly labeled" requirement without broken links.

---

### Step 2 — Fix the Getting Started section (lines 95–130)

**Problem areas to address:**

1. **Unclosed code fence** — The Prerequisites block opens a ` ```bash ` fence but the Environment Variables subsection is inside it without a closing fence.

2. **Build command** — `cargo build --release --target wasm32-unknown-unknown` works but `stellar contract build` is the canonical command (used in `CONTRIBUTING.md` and the Makefile). Replace it.

3. **Malformed "Run the AI Agent" block** — `bashcd agent` is garbled markdown for ` ```bash\ncd agent `. This path doesn't exist. Remove the block entirely.

4. **Malformed "Run the Frontend" block** — Same problem, same fix: remove it.

**Replacement Getting Started section:**

````markdown
## Getting Started

### Prerequisites

Install Rust and the WASM target:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
```

Install the Stellar CLI (pinned to 21.2.0):
```bash
cargo install --locked stellar-cli --version 21.2.0 --features opt
```

### Environment Variables

Copy the template and add your secret key:
```bash
cp .env.devnet.template .env.devnet
# Edit .env.devnet and set SOROBAN_SECRET_KEY
```

### Build the Contract

```bash
cd neurowealth-vault
stellar contract build
```

The compiled WASM is output to `target/wasm32v1-none/release/neurowealth_vault.wasm`.

### Run Tests

```bash
cd neurowealth-vault
cargo test
```

### Deploy to Devnet

```bash
./scripts/deploy-devnet.sh
```

See [`scripts/README-E2E.md`](scripts/README-E2E.md) for end-to-end devnet validation.

> For the AI agent, frontend, and WhatsApp bot — see [Planned Components](#planned-components) above.
````

---

### Step 3 — Add cross-links

After the Getting Started section, add a short **"Further Reading"** section:

```markdown
## Further Reading

| Document | Purpose |
|----------|---------|
| [`ARCHITECTURE.md`](ARCHITECTURE.md) | Storage layout, share accounting math, asset flow diagrams |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | Development setup, CI requirements, PR process |
| [`scripts/README-E2E.md`](scripts/README-E2E.md) | End-to-end devnet test guide |
| [`SECURITY.md`](SECURITY.md) | Trust model and threat analysis |
| [`docs/MAINNET_CHECKLIST.md`](docs/MAINNET_CHECKLIST.md) | Pre-mainnet deployment checklist |
```

---

### Step 4 — Audit the rest of the README for stale paths

- The **Quick Start (Devnet)** section near the bottom also uses `cargo build --release --target wasm32-unknown-unknown`. Update it to `stellar contract build` for consistency.
- The **Mainnet** section already links to `docs/MAINNET_CHECKLIST.md` — no change needed.
- Verify no remaining `cd contracts` references (mentioned in the issue description).

---

## 4. Edge Cases, Constraints, and Potential Blockers

| Item | Notes |
|------|-------|
| **Tracking issue links** | Issue #230 asks for links to GitHub tracking issues for `agent/`, `frontend/`, `whatsapp/`. If those issues don't exist yet, the table satisfies "clearly labeled" without links — confirm before implementation |
| **Duplicate Quick Start section** | The README has both a short "Getting Started" (lines 95–130) and a longer "Quick Start (Devnet)" section (lines ~285–345). Both cover building and deploying. This plan makes them consistent; merging them is out of scope for #230 |
| **`cargo build` vs `stellar contract build`** | Both produce equivalent output. `stellar contract build` is the preferred form per `CONTRIBUTING.md` and the vault `Makefile`. This is a quality improvement, not a breaking change |
| **Scope** | Only `README.md` is modified. No Rust, no scripts, no other docs |

---

## 5. Verification

After implementation, verify with:
1. Follow only the README instructions (prerequisites → build → test → deploy-devnet) and confirm every command succeeds
2. Run `grep -n "cd contracts\|cd agent\|cd frontend\|soroban " README.md` — should return nothing
3. Render the README and confirm no broken code fences or malformed markdown
4. Confirm all relative links resolve: `ARCHITECTURE.md`, `CONTRIBUTING.md`, `scripts/README-E2E.md`
