#!/bin/bash
# WRAITH Protocol — Mainnet Deploy Script

set -e

PROGRAM_NAME="wraith_core"
CLUSTER="mainnet-beta"
KEYPAIR="${KEYPAIR:-~/.config/solana/id.json}"

echo "╔══════════════════════════════════════╗"
echo "║  WRAITH Protocol — Deploy v0.1       ║"
echo "╚══════════════════════════════════════╝"
echo ""
echo "[*] Network : $CLUSTER"
echo "[*] Keypair : $KEYPAIR"
echo ""

# Check tools
command -v solana >/dev/null 2>&1 || { echo "[!] solana-cli not found"; exit 1; }
command -v anchor  >/dev/null 2>&1 || { echo "[!] anchor not found";    exit 1; }

# Set cluster
solana config set --url $CLUSTER

echo "[*] Building program..."
anchor build

echo "[*] Deploying $PROGRAM_NAME..."
anchor deploy --program-name $PROGRAM_NAME

echo ""
echo "[✓] Deployment complete."
echo "[i] Verify at: https://solscan.io/account/$(solana address)"
