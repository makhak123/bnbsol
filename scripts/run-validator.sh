#!/bin/bash


set -e

echo "🔧 Starting BNB-Solana Bridge Validator"
echo "========================================"

# Check if config exists
if [ ! -f "config.json" ]; then
    echo "❌ config.json not found. Run ./scripts/deploy.sh first"
    exit 1
fi

# Load configuration
SOLANA_RPC=$(jq -r '.solana.rpc_url' config.json)
BNB_RPC=$(jq -r '.bnb.rpc_url' config.json)
PROGRAM_ID=$(jq -r '.solana.program_id' config.json)
CONTRACT_ADDRESS=$(jq -r '.bnb.contract_address' config.json)

# Check environment variables
if [ -z "$VALIDATOR_KEYPAIR" ]; then
    VALIDATOR_KEYPAIR="$HOME/.config/solana/id.json"
    echo "⚠️  Using default keypair: $VALIDATOR_KEYPAIR"
fi

if [ ! -f "$VALIDATOR_KEYPAIR" ]; then
    echo "❌ Keypair not found at $VALIDATOR_KEYPAIR"
    echo "Set VALIDATOR_KEYPAIR environment variable or create keypair:"
    echo "  solana-keygen new -o $VALIDATOR_KEYPAIR"
    exit 1
fi

# Check balances
echo ""
echo "💰 Checking balances..."
SOLANA_BALANCE=$(solana balance --url $SOLANA_RPC --keypair $VALIDATOR_KEYPAIR | awk '{print $1}')
echo "Solana balance: $SOLANA_BALANCE SOL"

if (( $(echo "$SOLANA_BALANCE < 0.1" | bc -l) )); then
    echo "⚠️  Low SOL balance. You may need more for transaction fees."
fi

# Build validator if needed
if [ ! -f "validator/target/release/validator" ]; then
    echo ""
    echo "🔨 Building validator..."
    cd validator
    cargo build --release
    cd ..
fi

# Start validator
echo ""
echo "🚀 Starting validator..."
echo "Solana RPC: $SOLANA_RPC"
echo "BNB RPC: $BNB_RPC"
echo "Program ID: $PROGRAM_ID"
echo "Contract: $CONTRACT_ADDRESS"
echo ""

./validator/target/release/validator \
  --solana-rpc "$SOLANA_RPC" \
  --bnb-rpc "$BNB_RPC" \
  --keypair-path "$VALIDATOR_KEYPAIR" \
  --bridge-program-id "$PROGRAM_ID" \
  --bnb-bridge-contract "$CONTRACT_ADDRESS" \
  --poll-interval 400

echo "✅ Validator stopped"
