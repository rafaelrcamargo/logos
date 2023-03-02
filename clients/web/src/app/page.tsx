import clsx from "clsx"

import { Suspense } from "react"
const Hello = dynamic(() => import("components/hello"), { ssr: false })

import { Inter } from "next/font/google"
import Loading from "app/loading"
import dynamic from "next/dynamic"
const inter = Inter({ variable: "--font-inter" })

export default function Home() {
  return (
    <main
      className={clsx(
        "flex min-h-screen w-screen flex-col items-center justify-center gap-8 bg-zinc-100 font-sans dark:bg-zinc-900",
        inter.variable
      )}
    >
      <Suspense fallback={<Loading />}>
        <Hello />
      </Suspense>
    </main>
  )
}
