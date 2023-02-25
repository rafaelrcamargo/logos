import type { Metadata } from "next"
import "styles/globals.css"

const base = {
  title: "Logos - A better news feed",
  description: "Logos is a new way to read the news.",
}

export const metadata: Metadata = {
  title: {
    default: base.title,
    template: "%s | Logos",
  },
  keywords: ["Next.js", "Rust", "GraphQL", "Logos", "News"],
  authors: [
    { name: "Rafael R. Camargo", url: "https://github.com/rafaelrcamargo" },
  ],
  creator: "Rafael R. Camargo",
  publisher: "Rafael R. Camargo",
  openGraph: {
    title: base.title,
    description: base.description,
    url: "https://logos.cmrg.dev",
    siteName: "Logos",
    images: [
      {
        url: "https://logos.cmrg.dev/og.png",
        width: 1200,
        height: 630,
      },
    ],
    locale: "en-US",
    type: "website",
  },
  twitter: {
    card: "summary_large_image",
    title: base.title,
    description: base.description,
    creator: "@rafaelrcamargo",
    images: ["https://logos.cmrg.dev/og.png"],
  },
  description: base.description,
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" className="antialiased">
      <head />
      <body>{children}</body>
    </html>
  )
}
