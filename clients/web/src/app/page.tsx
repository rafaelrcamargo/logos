import { cookies } from "next/headers"

import Auth from "components/auth"
import Welcome from "components/welcome"

export default function Home() {
  const Cookies = cookies()
  const session = Cookies.has("id")

  return (
    <main className={"center flex-col gap-8 font-sans"}>
      {session ? <Welcome /> : <Auth />}
    </main>
  )
}
