#!/bin/bash


set -e

echo "🚀 BNB-Solana Bridge Deployment Script"
echo "========================================"

# Check prerequisites
command -v anchor >/dev/null 2>&1 || { echo "❌ Anchor not installed. Run: cargo install --git https://github.com/coral-xyz/anchor avm"; exit 1; }
command -v forge >/dev/null 2>&1 || { echo "❌ Foundry not installed. Run: curl -L https://foundry.paradigm.xyz | bash"; exit 1; }
command -v solana >/dev/null 2>&1 || { echo "❌ Solana CLI not installed"; exit 1; }

# Parse arguments
NETWORK=${1:-devnet}

if [ "$NETWORK" != "devnet" ] && [ "$NETWORK" != "mainnet" ]; then
    echo "Usage: ./deploy.sh [devnet|mainnet]"
    exit 1
fi

echo "📍 Deploying to: $NETWORK"

# Deploy Solana program
echo ""
echo "📦 Building Solana program..."
cd bridge-program
anchor build

echo "🌐 Deploying to Solana $NETWORK..."
if [ "$NETWORK" = "devnet" ]; then
    PROGRAM_ID=$(anchor deploy --provider.cluster devnet | grep "Program Id:" | awk '{print $3}')
else
    PROGRAM_ID=$(anchor deploy --provider.cluster mainnet | grep "Program Id:" | awk '{print $3}')
fi

echo "✅ Solana Program ID: $PROGRAM_ID"

# Deploy BNB contract
echo ""
echo "📦 Deploying BNB Chain contract..."
cd ../contracts

if [ "$NETWORK" = "devnet" ]; then
    BNB_RPC="https://data-seed-prebsc-1-s1.binance.org:8545"
else
    BNB_RPC="https://bsc-dataseed.binance.org/"
fi

forge build
CONTRACT_ADDRESS=$(forge script script/Deploy.s.sol --rpc-url $BNB_RPC --broadcast --json | jq -r '.returns.bridge.value')

echo "✅ BNB Contract: $CONTRACT_ADDRESS"

# Build validator
echo ""
echo "🔧 Building validator..."
cd ../validator
cargo build --release

# Generate config
echo ""
echo "📝 Generating configuration..."
cd ..
cat > config.json <<EOF
{
  "network": "$NETWORK",
  "solana": {
    "rpc_url": "$([ "$NETWORK" = "devnet" ] && echo "https://api.devnet.solana.com" || echo "https://api.mainnet-beta.solana.com")",
    "ws_url": "$([ "$NETWORK" = "devnet" ] && echo "wss://api.devnet.solana.com" || echo "wss://api.mainnet-beta.solana.com")",
    "program_id": "$PROGRAM_ID"
  },
  "bnb": {
    "rpc_url": "$BNB_RPC",
    "contract_address": "$CONTRACT_ADDRESS",
    "chain_id": $([ "$NETWORK" = "devnet" ] && echo "97" || echo "56")
  },
  "validator": {
    "threshold": 2,
    "poll_interval_ms": 400
  }
}
EOF

echo "✅ Configuration saved to config.json"

echo ""
echo "🎉 Deployment Complete!"
echo "========================================"
echo "Solana Program: $PROGRAM_ID"
echo "BNB Contract: $CONTRACT_ADDRESS"
echo "Config: config.json"
echo ""
echo "Next steps:"
echo "1. Review config.json"
echo "2. Run validator: ./scripts/run-validator.sh"
echo "3. Test bridge: cd tests && npm test"
