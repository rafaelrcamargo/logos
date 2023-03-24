import { FC, PropsWithChildren } from "react"
import { Github, Discord, Spotify } from "components/icons"
import { cn } from "lib/utils"
import Image from "next/image"

const providers = {
  github: [Github, "fill-slate-800 dark:fill-slate-200"],
  discord: [Discord, "fill-indigo-500"],
  spotify: [Spotify, "fill-green-500"],
}

const Auth = () => (
  <div className="center max-h-screen w-full flex-col overflow-hidden lg:flex-row">
    <div className="w-full max-w-lg px-8 pb-16 pt-48">
      <h1 className="text-3xl md:text-5xl">Welcome!</h1>
      <p className="text-md md:text-xl">
        Please sign in with one of our providers:
      </p>
      <div className="flex flex-col gap-4 pt-8">
        {Object.entries(providers).map(([provider, [Icon, color]]) => (
          <Link
            key={provider}
            href={`http://localhost/api/v1/oauth/create?provider=${provider}`}
            className={cn(
              "center gap-4 rounded-lg border border-neutral-300 py-3 capitalize shadow-md shadow-neutral-300/10 duration-150 hover:scale-[1.025] hover:shadow-xl active:hover:scale-100 dark:border-neutral-600/30 dark:shadow-neutral-900",
              color
            )}
          >
            <span className="center h-6 w-6 md:h-8 md:w-8">
              <Icon />
            </span>
            <span className="text-md md:text-xl">{provider}</span>
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

export default Auth
