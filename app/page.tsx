import { Hero } from "@/components/hero"
import { TechnicalExplanation } from "@/components/technical-explanation"
import { CodeRepository } from "@/components/code-repository"
import { Footer } from "@/components/footer"

export default function Page() {
  return (
    <main className="min-h-screen relative overflow-hidden grid-bg">
      <div className="scanline" />
      <Hero />
      <TechnicalExplanation />
      <CodeRepository />
      <Footer />
    </main>
  )
}
