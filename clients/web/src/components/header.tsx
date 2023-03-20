import { Library } from "lucide-react"
import Image from "next/image"

const getUser = async () => {
  const res = await fetch(
    `http://127.0.0.1:8082/api/v1/user/5e063240-2afd-4b95-a283-8174b268d75b`,
    { cache: "force-cache" }
  )

  if (!res.ok) {
    // This will activate the closest `error.js` Error Boundary
    throw new Error("Failed to fetch user data")
  }

  return res.json() as Promise<{
    email: string
    username: string
    image: string
    verified: boolean
  }>
}

export const Header = async () => {
  const user = await getUser()

  return (
    <header className="fixed inset-0 flex h-16 border-b border-neutral-600/10 bg-neutral-900/10 px-8 shadow-lg backdrop-blur-md">
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

        <Image
          src={user.image}
          alt={user.username}
          width={32}
          height={32}
          loading="lazy"
          className="rounded-full"
        />
      </div>
    </header>
  )
}
