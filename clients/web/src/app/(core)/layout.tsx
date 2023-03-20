import { FC, PropsWithChildren } from "react"

const Layout: FC<PropsWithChildren> = ({ children }) => {
  return (
    <>
      {/* <Header /> */}

      <main className="center m-auto min-h-screen w-screen px-8">
        {children}
      </main>
    </>
  )
}

export default Layout
