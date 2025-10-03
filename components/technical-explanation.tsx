"use client"

import { Card } from "@/components/ui/card"
import { useEffect, useRef, useState } from "react"

const sections = [
  {
    title: "BRIDGE ARCHITECTURE",
    content: `Production-ready bridge using Anchor framework on Solana and Foundry contracts on BNB Chain. Validators monitor both chains simultaneously, detecting lock/burn events and submitting cryptographic proofs. No centralized custodians - all assets secured by multi-signature validator consensus.`,
    code: `// Actual Solana program structure
#[program]
pub mod bnb_solana_bridge {
    pub fn bridge_from_bnb(
        ctx: Context<BridgeFromBnb>,
        amount: u64,
        bnb_tx_hash: [u8; 32],
        validator_signatures: Vec<[u8; 64]>,
    ) -> Result<()>
}`,
  },
  {
    title: "VALIDATOR CONSENSUS",
    content: `Rust-based validator nodes run 400ms polling loops on both chains. When tokens are locked on BNB Chain, validators sign the event and submit proofs to Solana. Multi-signature threshold (configurable, default 2/3) ensures security. Validators stake reputation and can be slashed for malicious behavior.`,
    code: `// Real validator implementation
pub struct BridgeValidator {
    solana_client: Arc<RpcClient>,
    bnb_provider: Arc<Provider<Http>>,
    validator_keypair: Arc<Keypair>,
}

async fn monitor_bnb_locks(&self) -> Result<()> {
    // Detect lock events and submit to Solana
}`,
  },
  {
    title: "SECURITY MODEL",
    content: `Replay attack prevention using processed transaction tracking. Each BNB transaction can only mint once on Solana. Emergency pause mechanism allows governance to halt bridge during incidents. All validator signatures verified on-chain. Open source code enables community audits.`,
    code: `// Replay protection
#[account]
pub struct ProcessedTx {
    pub is_processed: bool,
    pub bnb_tx_hash: [u8; 32],
    pub timestamp: i64,
}`,
  },
  {
    title: "TOKEN MECHANICS",
    content: `BEP-20 tokens locked in BNB smart contract trigger SPL token minting on Solana via Program Derived Addresses (PDAs). Burning SPL tokens emits events that validators sign to unlock original tokens on BNB Chain. 1:1 peg maintained through cryptographic proofs, not price oracles.`,
    code: `// Token bridge flow
pub fn bridge_to_bnb(
    ctx: Context<BridgeToBnb>,
    amount: u64,
    bnb_recipient: [u8; 20],
) -> Result<()> {
    token::burn(cpi_ctx, amount)?;
    emit!(BridgeToBnbEvent { ... });
    Ok(())
}`,
  },
]

function TypingText({ text, delay = 20 }: { text: string; delay?: number }) {
  const [displayText, setDisplayText] = useState("")
  const [isVisible, setIsVisible] = useState(false)
  const ref = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setIsVisible(true)
        }
      },
      { threshold: 0.1 },
    )

    if (ref.current) {
      observer.observe(ref.current)
    }

    return () => observer.disconnect()
  }, [])

  useEffect(() => {
    if (!isVisible) return

    let index = 0
    const timer = setInterval(() => {
      if (index <= text.length) {
        setDisplayText(text.slice(0, index))
        index++
      } else {
        clearInterval(timer)
      }
    }, delay)

    return () => clearInterval(timer)
  }, [text, delay, isVisible])

  return <div ref={ref}>{displayText}</div>
}

export function TechnicalExplanation() {
  return (
    <section id="technical" className="container mx-auto px-4 py-20">
      <div className="max-w-6xl mx-auto space-y-12">
        <div className="text-center space-y-4">
          <h2 className="text-3xl md:text-5xl font-bold text-primary crt-effect">{">"} HOW IT WORKS</h2>
          <p className="text-muted-foreground text-sm md:text-base">
            Production bridge architecture and implementation
          </p>
        </div>

        <div className="grid gap-8">
          {sections.map((section, index) => (
            <Card
              key={index}
              className="border-primary/30 bg-card/50 backdrop-blur-sm p-6 md:p-8 hover:border-primary/50 transition-all hover:shadow-[0_0_30px_rgba(0,255,0,0.1)] animate-fade-in-up"
              style={{ animationDelay: `${index * 100}ms` }}
            >
              <h3 className="text-xl md:text-2xl font-bold text-secondary mb-4 flex items-center gap-2">
                <span className="text-primary">{">"}</span>
                {section.title}
              </h3>

              <div className="space-y-4">
                <p className="text-card-foreground leading-relaxed text-sm md:text-base">
                  <TypingText text={section.content} delay={5} />
                </p>

                <div className="bg-background/80 border border-primary/20 rounded p-4 overflow-x-auto">
                  <pre className="text-xs md:text-sm text-primary">
                    <code>{section.code}</code>
                  </pre>
                </div>
              </div>
            </Card>
          ))}
        </div>

        <Card className="border-secondary/30 bg-card/50 backdrop-blur-sm p-6 md:p-8 mt-12">
          <h3 className="text-xl md:text-2xl font-bold text-secondary mb-4 flex items-center gap-2">
            <span className="text-primary">{">"}</span>
            PERFORMANCE METRICS
          </h3>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4 md:gap-6">
            <div className="text-center">
              <div className="text-3xl md:text-4xl font-bold text-primary crt-effect">65K+</div>
              <div className="text-xs md:text-sm text-muted-foreground mt-2">TPS</div>
            </div>
            <div className="text-center">
              <div className="text-3xl md:text-4xl font-bold text-primary crt-effect">{"<"}400ms</div>
              <div className="text-xs md:text-sm text-muted-foreground mt-2">Finality</div>
            </div>
            <div className="text-center">
              <div className="text-3xl md:text-4xl font-bold text-primary crt-effect">$0.00001</div>
              <div className="text-xs md:text-sm text-muted-foreground mt-2">Tx Cost</div>
            </div>
            <div className="text-center">
              <div className="text-3xl md:text-4xl font-bold text-primary crt-effect">100%</div>
              <div className="text-xs md:text-sm text-muted-foreground mt-2">Uptime</div>
            </div>
          </div>
        </Card>
      </div>
    </section>
  )
}
