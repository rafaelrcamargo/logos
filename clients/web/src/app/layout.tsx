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
      <body className="center min-w-screen m-auto min-h-screen max-w-4xl font-sans antialiased">
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
