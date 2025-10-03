"use client"

import { Card } from "@/components/ui/card"

const files = [
  {
    name: "bridge-program/src/lib.rs",
    description: "Production Solana program with full bridge logic",
    language: "rust",
  },
  {
    name: "validator/src/main.rs",
    description: "Working validator node for cross-chain monitoring",
    language: "rust",
  },
  {
    name: "contracts/BnbBridge.sol",
    description: "Production BNB Chain bridge contract",
    language: "solidity",
  },
  {
    name: "README.md",
    description: "Complete setup and deployment guide",
    language: "markdown",
  },
  {
    name: "scripts/deploy.sh",
    description: "One-command deployment script",
    language: "bash",
  },
]

export function CodeRepository() {
  return (
    <section id="repository" className="container mx-auto px-4 py-20">
      <div className="max-w-7xl mx-auto space-y-8">
        <div className="text-center space-y-4">
          <h2 className="text-3xl md:text-5xl font-bold text-primary crt-effect">{">"} PRODUCTION CODE</h2>
          <p className="text-muted-foreground text-sm md:text-base">Full working implementation - clone and deploy</p>
        </div>

        <Card className="border-secondary/30 bg-card/50 backdrop-blur-sm p-6 md:p-8">
          <h3 className="text-xl md:text-2xl font-bold text-secondary mb-4 flex items-center gap-2">
            <span className="text-primary">{">"}</span>
            GITHUB REPOSITORY
          </h3>
          <div className="space-y-4">
            <div className="bg-background/80 border border-primary/20 rounded p-4">
              <pre className="text-xs md:text-sm text-primary">
                <code>{`# Clone the production repository
git clone https://github.com/makhak123/v0-bnb-on-solana
cd v0-bnb-on-solana

# One-command deployment
chmod +x scripts/deploy.sh
./scripts/deploy.sh

# Start validator node
chmod +x scripts/run-validator.sh
./scripts/run-validator.sh

# Repository includes:
# ✅ Full Anchor Solana program (Rust)
# ✅ Cross-chain validator (Rust + Tokio)
# ✅ BNB Chain contracts (Solidity + Foundry)
# ✅ Deployment scripts
# ✅ Integration tests
# ✅ Complete documentation`}</code>
              </pre>
            </div>

            <div className="grid md:grid-cols-2 gap-4 mt-6">
              <div className="bg-background/60 border border-primary/20 rounded p-4">
                <h4 className="text-secondary font-bold mb-2">{">"} FEATURES</h4>
                <ul className="text-sm text-muted-foreground space-y-1">
                  <li>• Trustless validator consensus</li>
                  <li>• Replay attack prevention</li>
                  <li>• Emergency pause mechanism</li>
                  <li>• Multi-sig security</li>
                  <li>• Production-ready code</li>
                </ul>
              </div>

              <div className="bg-background/60 border border-primary/20 rounded p-4">
                <h4 className="text-secondary font-bold mb-2">{">"} TECH STACK</h4>
                <ul className="text-sm text-muted-foreground space-y-1">
                  <li>• Anchor Framework 0.29</li>
                  <li>• Solana Web3.js</li>
                  <li>• Ethers.rs for BNB Chain</li>
                  <li>• Foundry for Solidity</li>
                  <li>• Tokio async runtime</li>
                </ul>
              </div>
            </div>

            <p className="text-sm text-muted-foreground mt-4">
              <span className="text-secondary">{">"}</span> All code is production-ready and includes comprehensive
              tests. Deploy to devnet first, then mainnet after security audit.
            </p>
          </div>
        </Card>
      </div>
    </section>
  )
}
