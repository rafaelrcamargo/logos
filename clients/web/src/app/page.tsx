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
        "font-sans w-screen min-h-screen flex flex-col gap-8 items-center justify-center dark:bg-zinc-900 bg-zinc-100",
        inter.variable
      )}
    >
      <Suspense fallback={<Loading />}>
        <Hello />
      </Suspense>
    </main>
  )
}
