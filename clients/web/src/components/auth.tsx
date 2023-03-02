import { FC, PropsWithChildren } from "react"
import { cn } from "lib/utils"

import { Github, Discord, Spotify } from "components/icons"

const providers = {
  github: [Github, "fill-slate-200"],
  discord: [Discord, "fill-indigo-500"],
  spotify: [Spotify, "fill-green-500"],
}

export default () => (
  <>
    <h1 className="text-6xl">Sign in:</h1>
    <div className="flex flex-row gap-4">
      {Object.entries(providers).map(([provider, [Icon, color]]) => (
        <Link
          key={provider}
          href={`http://127.0.0.1:8081/api/v1/oauth/create?provider=${provider}`}
          className={cn(
            "center h-12 w-12 shadow-2xl duration-150 hover:opacity-50",
            color
          )}
        >
          <Icon />
        </Link>
      ))}
    </div>
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
