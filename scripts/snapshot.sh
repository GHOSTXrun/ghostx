#!/bin/bash
# WRAITH Protocol — Holder Snapshot Script
# Takes a snapshot of all $WRAITH holders for governance / reward distribution

set -e

MINT_ADDRESS="${1:-WRAiTHxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx}"
OUTPUT="snapshot_$(date +%Y%m%d_%H%M%S).json"

echo "[*] Taking holder snapshot for: $MINT_ADDRESS"
echo "[*] Output: $OUTPUT"

spl-token accounts --owner $MINT_ADDRESS --output json > $OUTPUT

echo "[✓] Snapshot saved to $OUTPUT"
echo "[i] Total holders: $(cat $OUTPUT | python3 -c 'import json,sys; d=json.load(sys.stdin); print(len(d.get("accounts",[])))')"
