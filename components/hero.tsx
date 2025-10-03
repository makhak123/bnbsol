"use client"

import { useEffect, useState } from "react"
import Image from "next/image"

export function Hero() {
  const [text, setText] = useState("")
  const fullText = "> INITIALIZING BNB-SOLANA BRIDGE PROTOCOL..."

  useEffect(() => {
    let index = 0
    const timer = setInterval(() => {
      if (index <= fullText.length) {
        setText(fullText.slice(0, index))
        index++
      } else {
        clearInterval(timer)
      }
    }, 50)

    return () => clearInterval(timer)
  }, [])

  return (
    <section className="container mx-auto px-4 py-20 min-h-screen flex flex-col justify-center items-center relative">
      <div className="absolute inset-0 bg-gradient-to-b from-primary/5 via-transparent to-transparent pointer-events-none" />

      <div className="text-center space-y-8 relative z-10">
        <div className="flex justify-center mb-8 animate-pulse">
          <div className="relative w-32 h-32 md:w-40 md:h-40">
            <Image src="/bnb-logo.png" alt="BNB Logo" fill className="object-contain crt-effect" />
          </div>
        </div>

        <div className="space-y-4">
          <h1 className="text-4xl md:text-6xl lg:text-7xl font-bold text-primary crt-effect animate-fade-in">
            BNB × SOLANA
          </h1>
          <div className="h-8 md:h-10">
            <p className="text-lg md:text-xl text-muted-foreground terminal-cursor">{text}</p>
          </div>
        </div>

        <div className="border border-primary/30 bg-card/50 backdrop-blur-sm p-6 md:p-8 rounded-lg max-w-3xl mx-auto mt-12 animate-fade-in-up">
          <p className="text-sm md:text-base text-card-foreground leading-relaxed text-left font-mono">
            <span className="text-secondary">{">"}</span> SYSTEM MESSAGE: Following directive from @aeyakovenko
            <br />
            <span className="text-secondary">{">"}</span> OBJECTIVE: Fork BNB Chain and deploy on Solana infrastructure
            <br />
            <span className="text-secondary">{">"}</span> STATUS:{" "}
            <span className="text-primary animate-pulse">PROTOCOL ACTIVE</span>
          </p>
        </div>

        <div className="flex flex-col sm:flex-row gap-4 justify-center mt-12">
          <a
            href="#technical"
            className="px-8 py-3 bg-primary text-primary-foreground rounded hover:bg-primary/90 transition-all hover:shadow-[0_0_20px_rgba(0,255,0,0.3)] font-mono text-sm md:text-base"
          >
            {">"} VIEW TECHNICAL SPECS
          </a>
          <a
            href="https://github.com/makhak123/v0-bnb-on-solana"
            target="_blank"
            rel="noopener noreferrer"
            className="px-8 py-3 border border-primary text-primary rounded hover:bg-primary/10 transition-all hover:shadow-[0_0_20px_rgba(0,255,0,0.2)] font-mono text-sm md:text-base"
          >
            {">"} ACCESS REPOSITORY
          </a>
        </div>

        <div className="absolute bottom-10 left-1/2 -translate-x-1/2 animate-bounce">
          <div className="text-primary/50 text-2xl">▼</div>
        </div>
      </div>
    </section>
  )
}
