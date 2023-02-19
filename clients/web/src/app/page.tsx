import clsx from "clsx"

import { Inter } from '@next/font/google'
const inter = Inter({ variable: "--font-inter" })

export default function Home () {
  return (
    <main className={clsx("font-sans w-screen min-h-screen flex items-center justify-center dark:bg-zinc-900 bg-zinc-100", inter.variable)}>
      <h1 className="font-black text-6xl dark:text-zinc-100 text-zinc-900">
        Hello from Next.js!
      </h1>
    </main>
  )
}
