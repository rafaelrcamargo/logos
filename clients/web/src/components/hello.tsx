"use client"

import { Github, Discord, Spotify } from "components/icons"
import { cn } from "lib/utils"
import { FC, PropsWithChildren } from "react"

const providers = {
  github: [Github, "fill-slate-200"],
  discord: [Discord, "fill-indigo-500"],
  spotify: [Spotify, "fill-green-500"],
}

export default () => (
  <>
    {document.cookie.includes("id") ? (
      <h1 className="text-6xl font-black text-zinc-900 dark:text-zinc-100">
        Welcome to Logos!
      </h1>
    ) : (
      <>
        <h1 className="text-6xl font-black text-zinc-900 dark:text-zinc-100">
          Sign in:
        </h1>
        <div className="flex flex-row gap-4">
          {Object.entries(providers).map(([provider, [Icon, color]]) => (
            <Link
              key={provider}
              href={`http://127.0.0.1:8081/api/v1/oauth/create?provider=${provider}`}
              className={cn(
                "flex h-12 w-12 items-center shadow-2xl duration-150 hover:opacity-50",
                color
              )}
            >
              <Icon />
            </Link>
          ))}
        </div>
      </>
    )}
  </>
)

type Props = FC<PropsWithChildren<{ href: string; className: string }>>
const Link: Props = ({ href, className, children }) => {
  return (
    <a href={href} className={className}>
      {children}
    </a>
  )
}
