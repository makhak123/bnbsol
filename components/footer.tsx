export function Footer() {
  return (
    <footer className="border-t border-primary/30 bg-card/30 backdrop-blur-sm py-12">
      <div className="container mx-auto px-4">
        <div className="max-w-6xl mx-auto">
          <div className="grid md:grid-cols-3 gap-8 mb-8">
            <div>
              <h3 className="text-lg font-bold text-primary mb-4">{">"} PROJECT</h3>
              <ul className="space-y-2 text-sm text-muted-foreground">
                <li>
                  <a href="#technical" className="hover:text-primary transition-colors">
                    Technical Docs
                  </a>
                </li>
                <li>
                  <a href="#repository" className="hover:text-primary transition-colors">
                    Repository
                  </a>
                </li>
                <li>
                  <a href="https://github.com" className="hover:text-primary transition-colors">
                    GitHub
                  </a>
                </li>
              </ul>
            </div>

            <div>
              <h3 className="text-lg font-bold text-primary mb-4">{">"} RESOURCES</h3>
              <ul className="space-y-2 text-sm text-muted-foreground">
                <li>
                  <a href="https://solana.com" className="hover:text-primary transition-colors">
                    Solana Docs
                  </a>
                </li>
                <li>
                  <a href="https://docs.bnbchain.org" className="hover:text-primary transition-colors">
                    BNB Chain Docs
                  </a>
                </li>
                <li>
                  <a href="https://www.anchor-lang.com" className="hover:text-primary transition-colors">
                    Anchor Framework
                  </a>
                </li>
              </ul>
            </div>

            <div>
              <h3 className="text-lg font-bold text-primary mb-4">{">"} COMMUNITY</h3>
              <ul className="space-y-2 text-sm text-muted-foreground">
                <li>
                  <a href="https://twitter.com/aeyakovenko" className="hover:text-primary transition-colors">
                    @aeyakovenko
                  </a>
                </li>
                <li>
                  <a href="https://discord.gg/solana" className="hover:text-primary transition-colors">
                    Discord
                  </a>
                </li>
                <li>
                  <a href="https://t.me/solana" className="hover:text-primary transition-colors">
                    Telegram
                  </a>
                </li>
              </ul>
            </div>
          </div>

          <div className="border-t border-primary/20 pt-8 text-center">
            <p className="text-sm text-muted-foreground font-mono">
              <span className="text-primary">{">"}</span> SYSTEM STATUS:{" "}
              <span className="text-primary animate-pulse">ONLINE</span>
            </p>
            <p className="text-xs text-muted-foreground mt-2">
              Inspired by @aeyakovenko's vision • Built with Solana & BNB Chain
            </p>
          </div>
        </div>
      </div>
    </footer>
  )
}
