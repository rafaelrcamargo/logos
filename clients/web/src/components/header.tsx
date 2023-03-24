import Image from "next/image"

import { Button } from "components/ui/button"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuPortal,
  DropdownMenuSeparator,
  DropdownMenuShortcut,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from "components/ui/dropdown-menu"

import {
  CreditCard,
  Keyboard,
  Library,
  LogOut,
  Mail,
  MessageSquare,
  PlusCircle,
  Settings,
  User,
  UserPlus,
  Users,
} from "lucide-react"
import { FC } from "react"
import Link from "next/link"

type User = {
  email: string
  username: string
  image: string
  verified: boolean
}

const logError = (err: any) => {
  console.error("Error: ", JSON.stringify(err))
  return undefined
}

type Cookie = {
  /** A string with the name of a cookie. */
  name: string
  /** A string containing the value of the cookie. */
  value: string
}

const getUser = async (session: string | undefined) => {
  if (!session) return undefined

  const res = await fetch(`http://localhost/api/v1/user`, {
    headers: {
      cookie: `id=${session}`,
    },
  })

  if (!res.ok) return logError(res.status)

  const json = await res.json()
  if (json?.error) return logError(json.error)

  try {
    return json as User
  } catch (e) {
    return logError(e)
  }
}

import { cookies } from "next/headers"

export const Header = async () => {
  const session = cookies().get("id")?.value
  const user = await getUser(session)

  return (
    <header className="fixed inset-0 flex h-16 w-screen border-b border-neutral-300/50 bg-neutral-100/30 px-8 shadow-lg backdrop-blur-md dark:border-neutral-800/50 dark:bg-neutral-900/30">
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

const Menu: FC<{ user: User }> = ({ user }) => (
  <DropdownMenu>
    <DropdownMenuTrigger asChild>
      <Button
        variant="ghost"
        className="relative rounded-full p-0 focus:ring-0 focus:ring-offset-0"
      >
        <Image
          src={user.image}
          alt={user.username + "'s profile picture"}
          width={36}
          height={36}
          loading="lazy"
          className="rounded-full shadow-md"
        />
      </Button>
    </DropdownMenuTrigger>
    <DropdownMenuContent className="w-56" align="end" forceMount>
      <DropdownMenuLabel>My Account</DropdownMenuLabel>
      <DropdownMenuSeparator />
      <DropdownMenuGroup>
        <DropdownMenuItem>
          <User className="mr-2 h-4 w-4" />
          <span>Profile</span>
          <DropdownMenuShortcut>⇧⌘P</DropdownMenuShortcut>
        </DropdownMenuItem>
        <DropdownMenuItem>
          <CreditCard className="mr-2 h-4 w-4" />
          <span>Billing</span>
          <DropdownMenuShortcut>⌘B</DropdownMenuShortcut>
        </DropdownMenuItem>
        <DropdownMenuItem>
          <Settings className="mr-2 h-4 w-4" />
          <span>Settings</span>
          <DropdownMenuShortcut>⌘S</DropdownMenuShortcut>
        </DropdownMenuItem>
        <DropdownMenuItem>
          <Keyboard className="mr-2 h-4 w-4" />
          <span>Keyboard shortcuts</span>
          <DropdownMenuShortcut>⌘K</DropdownMenuShortcut>
        </DropdownMenuItem>
      </DropdownMenuGroup>
      <DropdownMenuSeparator />
      <DropdownMenuGroup>
        <DropdownMenuItem>
          <Users className="mr-2 h-4 w-4" />
          <span>Team</span>
        </DropdownMenuItem>
        <DropdownMenuSub>
          <DropdownMenuSubTrigger>
            <UserPlus className="mr-2 h-4 w-4" />
            <span>Invite users</span>
          </DropdownMenuSubTrigger>
          <DropdownMenuPortal>
            <DropdownMenuSubContent forceMount>
              <DropdownMenuItem>
                <Mail className="mr-2 h-4 w-4" />
                <span>Email</span>
              </DropdownMenuItem>
              <DropdownMenuItem>
                <MessageSquare className="mr-2 h-4 w-4" />
                <span>Message</span>
              </DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem>
                <PlusCircle className="mr-2 h-4 w-4" />
                <span>More...</span>
              </DropdownMenuItem>
            </DropdownMenuSubContent>
          </DropdownMenuPortal>
        </DropdownMenuSub>
      </DropdownMenuGroup>
      <DropdownMenuSeparator />
      <DropdownMenuItem>
        <LogOut className="mr-2 h-4 w-4 text-red-500/80 dark:text-red-500/60" />
        <span className="text-red-500/80 dark:text-red-500/60">Log out</span>
        <DropdownMenuShortcut className="text-red-500/80 dark:text-red-500/60">
          ⇧⌘Q
        </DropdownMenuShortcut>
      </DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>
)
