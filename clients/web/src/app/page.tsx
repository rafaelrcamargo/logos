"use client"

import clsx from "clsx"

import { Inter } from "@next/font/google"
import Link from "next/link"
import { useEffect, useState } from "react"
const inter = Inter({ variable: "--font-inter" })

export default function Home() {
  const [session, setSession] = useState<boolean>(false)

  useEffect(() => {
    if (document.cookie.includes("id")) {
      setSession(true)
    }
  }, [])

  return (
    <main
      className={clsx(
        "font-sans w-screen min-h-screen flex flex-col gap-8 items-center justify-center dark:bg-zinc-900 bg-zinc-100",
        inter.variable
      )}
    >
      {session ? (
        <h1 className="font-black text-6xl dark:text-zinc-100 text-zinc-900">
          Welcome to Logos!
        </h1>
      ) : (
        <>
          <h1 className="font-black text-6xl dark:text-zinc-100 text-zinc-900">
            Sign in:
          </h1>
          <div className="flex flex-col gap-4">
            <Link
              href={"http://127.0.0.1:8081/api/v1/oauth/github/create"}
              className="text-xl font-bold text-blue-500 underline"
            >
              Github
            </Link>
          </div>
        </>
      )}
    </main>
  )
}
