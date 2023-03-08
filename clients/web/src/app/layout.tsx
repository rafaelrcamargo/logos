import { FC, PropsWithChildren } from "react"

import type { Metadata } from "next"
import Font from "next/font/local"

import { cn } from "lib/utils"
import "styles/globals.css"

const sentient = Font({
  src: "../../public/fonts/Sentient.ttf",
  adjustFontFallback: "Times New Roman",
  variable: "--font-serif",
  display: "swap",
})

const satoshi = Font({
  src: "../../public/fonts/Satoshi.ttf",
  adjustFontFallback: "Arial",
  variable: "--font-sans",
  display: "swap",
})

const Header = () => (
  <header className="fixed inset-0 z-50 flex h-16 w-screen flex-row items-center justify-between border-b border-zinc-300/30 px-24 backdrop-blur-md backdrop-saturate-150">
    <a href="/" className="text-2xl font-black">
      <h1>Logos</h1>
    </a>
    <nav className="center gap-4">
      <a href="#" className="text-lg">
        Home
      </a>
      <a href="#" className="text-lg">
        About
      </a>
      <a href="#" className="text-lg">
        Contact
      </a>

      {/*
        TODO: Add a sign in button that redirects to the auth page and then back to the home page.
        Using a client component and a loading fallback.

        https://beta.nextjs.org/docs/routing/linking-and-navigating
      */}

      <a
        href={"/auth"}
        className="center h-10 rounded-full bg-zinc-900 px-6 text-lg text-zinc-50 dark:bg-zinc-50 dark:text-zinc-900"
      >
        Sign in
      </a>
    </nav>
  </header>
)

const RootLayout: FC<PropsWithChildren> = ({ children }) => {
  return (
    <html
      lang="en"
      className={cn(
        "bg-zinc-100 text-zinc-900 dark:bg-zinc-900 dark:text-zinc-100",
        sentient.variable,
        satoshi.variable
      )}
    >
      <body className="center m-auto min-h-screen w-screen font-sans antialiased">
        <Header />
        {children}
      </body>
    </html>
  )
}

export default RootLayout

const base = {
  title: "Logos - A better news feed",
  description: "Logos is a new way to read the news.",
}

export const metadata: Metadata = {
  title: {
    default: base.title,
    template: "%s | Logos",
  },
  description: base.description,
  keywords: ["Next.js", "Rust", "GraphQL", "Logos", "News"],
  authors: [
    { name: "Rafael R. Camargo", url: "https://github.com/rafaelrcamargo" },
  ],
  openGraph: {
    title: base.title,
    description: base.description,
    url: "https://logos.cmrg.dev",
    siteName: base.title,
    images: [
      {
        url: "https://logos.cmrg.dev/og.jpg",
        width: 1920,
        height: 1080,
      },
    ],
    locale: "en-US",
    type: "website",
  },
  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
      "max-video-preview": -1,
      "max-image-preview": "large",
      "max-snippet": -1,
    },
  },
  twitter: {
    title: base.title,
    card: "summary_large_image",
  },
  icons: {
    shortcut: "/favicon.ico",
  },
}
