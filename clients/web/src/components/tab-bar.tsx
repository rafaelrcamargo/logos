import { Home, Search, User } from "lucide-react"
import { FC } from "react"

export const TabBar: FC<{}> = ({}) => (
  <div className="glass fixed bottom-0 left-0 z-50 flex h-16 w-screen items-center justify-between gap-4 border-t p-2 px-16 md:hidden">
    <a href="/">
      <Home width={28} height={28} />
    </a>
    <a href="/">
      <Search width={28} height={28} />
    </a>
    <a href="/">
      <User width={28} height={28} />
    </a>
  </div>
)
