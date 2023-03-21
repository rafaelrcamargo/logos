import { FC, PropsWithChildren } from "react"
import { cn } from "lib/utils"

import { Github, Discord, Spotify } from "components/icons"
import Image from "next/image"

const providers = {
  github: [Github, "fill-slate-800 dark:fill-slate-200"],
  discord: [Discord, "fill-indigo-500"],
  spotify: [Spotify, "fill-green-500"],
}

export default () => (
  <div className="center max-h-screen w-full flex-col overflow-hidden lg:flex-row">
    <div className="w-full px-12 py-12 lg:w-2/5 lg:px-36">
      <h1 className="text-5xl">Welcome!</h1>
      <p className="text-xl">Please sign in with one of our providers:</p>
      <div className="flex flex-col gap-4 pt-8">
        {Object.entries(providers).map(([provider, [Icon, color]]) => (
          <Link
            key={provider}
            href={`http://127.0.0.1:8081/api/v1/oauth/create?provider=${provider}`}
            className={cn(
              "center gap-4 rounded-lg border border-neutral-300 py-3 capitalize shadow-md shadow-neutral-300/10 duration-150 hover:scale-[1.025] hover:shadow-xl active:hover:scale-100 dark:border-neutral-600/30 dark:shadow-neutral-900",
              color
            )}
          >
            <span className="center h-8 w-8">
              <Icon />
            </span>
            <span className="text-xl">{provider}</span>
          </Link>
        ))}
      </div>
    </div>
    <div className="relative -mb-[40%] h-screen w-screen overflow-hidden rounded-[2rem] object-none shadow-2xl shadow-neutral-300/50 dark:shadow-neutral-900 lg:mb-0 lg:-mr-[20%] lg:w-4/5">
      <Image
        fill
        src="/auth.webp"
        alt="Abstract stock image"
        className="object-cover"
      />
    </div>
  </div>
)

type Props = FC<PropsWithChildren<{ href: string; className: string }>>
const Link: Props = ({ href, className, children }) => {
  return (
    <a href={href} className={className}>
      {children}
    </a>
  )
}
