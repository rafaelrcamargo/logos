import { FC, PropsWithChildren } from "react"
import type { Metadata } from "next"
import Font from "next/font/local"
import { cn } from "lib/utils"

import "styles/globals.css"

const sentient = Font({
  src: "../../public/fonts/Sentient.woff2",
  adjustFontFallback: "Times New Roman",
  variable: "--font-serif",
  display: "swap",
  preload: true,
})

const satoshi = Font({
  src: "../../public/fonts/Satoshi.woff2",
  adjustFontFallback: "Arial",
  variable: "--font-sans",
  display: "swap",
  preload: true,
})

const Layout: FC<PropsWithChildren> = ({ children }) => {
  return (
    <html
      lang="en"
      className={cn(
        "overflow-x-hidden bg-neutral-100 text-neutral-900 dark:bg-neutral-900 dark:text-neutral-100",
        sentient.variable,
        satoshi.variable
      )}
    >
      <body className="font-sans antialiased">{children}</body>
    </html>
  )
}

export default Layout

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
        url: "/og.webp",
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
