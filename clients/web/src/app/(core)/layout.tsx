import { FC, PropsWithChildren } from "react"
import { Header } from "components/header"
import { TabBar } from "components/tab-bar"

const Layout: FC<PropsWithChildren> = ({ children }) => {
  return (
    <>
      {/* @ts-expect-error Async Server Component */}
      <Header />

      <main className="center m-auto min-h-screen w-screen px-8">
        {children}
      </main>

      <TabBar />
    </>
  )
}

export default Layout
