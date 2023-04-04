import type { User } from "types/api"

import { cookies } from "next/headers"
import Link from "next/link"

import { Button } from "components/ui/button"
import { Menu } from "components/ui/menu"
import { Library } from "lucide-react"
import { get } from "lib/utils"

export const Header = async () => {
  const session = cookies().get("id")?.value
  const user = session ? await get<User>("/user", session) : undefined

  return (
    <header className="glass fixed inset-0 flex h-16 w-screen border-b px-8">
      <div className="m-auto flex w-full max-w-4xl items-center justify-between">
        <a href="/" className="center gap-2 text-3xl">
          <Library />
          <h1>Logos</h1>
        </a>

        <nav className="hidden gap-8 md:flex">
          <a href="/">Home</a>
          <a href="/">Explore</a>
          <a href="/">About</a>
        </nav>

        {user ? (
          <Menu user={user} />
        ) : (
          <Link href="/auth">
            <Button variant="outline" className="rounded-full">
              Login
            </Button>
          </Link>
        )}
      </div>
    </header>
  )
}
