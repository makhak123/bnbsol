# BNB-Solana Bridge Deployment Guide

## Prerequisites

Install required tools:

\`\`\`bash
# Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Anchor Framework
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest

# Foundry (for Solidity)
curl -L https://foundry.paradigm.xyz | bash
foundryup
\`\`\`

## Environment Setup

Create a `.env` file:

\`\`\`bash
# Solana
VALIDATOR_KEYPAIR=/path/to/your/keypair.json

# BNB Chain
BNB_PRIVATE_KEY=your_private_key_here
BNB_RPC_URL=https://data-seed-prebsc-1-s1.binance.org:8545

# For mainnet
# BNB_RPC_URL=https://bsc-dataseed.binance.org/
\`\`\`

## Deployment Steps

### 1. Deploy to Devnet (Testing)

\`\`\`bash
# Make scripts executable
chmod +x scripts/*.sh

# Deploy everything
./scripts/deploy.sh devnet
\`\`\`

This will:
- Build and deploy the Solana program
- Deploy the BNB Chain bridge contract
- Generate a `config.json` with all addresses
- Build the validator node

### 2. Start Validator

\`\`\`bash
# Load environment variables
source .env

# Start the validator node
./scripts/run-validator.sh
\`\`\`

The validator will:
- Monitor BNB Chain for lock events
- Monitor Solana for burn events
- Submit cross-chain transactions
- Maintain consensus with other validators

### 3. Test the Bridge

\`\`\`bash
cd tests
npm install
npm test
\`\`\`

### 4. Deploy to Mainnet

**IMPORTANT**: Only deploy to mainnet after:
- Extensive devnet testing
- Professional security audit
- Multi-sig setup for admin functions

\`\`\`bash
# Deploy to mainnet
./scripts/deploy.sh mainnet

# Update validator config
./scripts/run-validator.sh
\`\`\`

## Architecture

\`\`\`
BNB Chain                    Validator Network                 Solana
┌─────────┐                 ┌──────────────┐                ┌─────────┐
│ Lock    │────────────────>│   Monitor    │───────────────>│  Mint   │
│ BNB     │                 │   Events     │                │  wBNB   │
└─────────┘                 └──────────────┘                └─────────┘
                                   │
                            ┌──────┴──────┐
                            │  Consensus  │
                            │  (2/3 sigs) │
                            └─────────────┘
\`\`\`

## Security Considerations

1. **Multi-sig Required**: Set up 3+ validators for production
2. **Rate Limiting**: Built-in limits prevent spam attacks
3. **Replay Protection**: Nonces prevent double-spending
4. **Emergency Pause**: Admin can pause bridge if issues detected
5. **Audit**: Get professional audit before mainnet deployment

## Monitoring

The validator logs all events:

\`\`\`bash
# View validator logs
tail -f validator.log

# Check bridge status
solana program show $PROGRAM_ID
\`\`\`

## Troubleshooting

**Deployment fails:**
- Check you have enough SOL/BNB for gas
- Verify RPC endpoints are accessible
- Ensure keypairs have correct permissions

**Validator not processing:**
- Check environment variables are set
- Verify both chains are synced
- Check validator has signing authority

## Support

For issues or questions:
- GitHub Issues: https://github.com/bnb-solana/bridge/issues
- Documentation: https://docs.bnb-solana.bridge
