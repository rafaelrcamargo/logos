import { Library } from "lucide-react"
import { FC, PropsWithChildren } from "react"

const Header = () => (
  <header className="fixed inset-0 z-50 flex h-20 w-screen flex-row items-center px-12">
    <a
      href="/"
      className="flex gap-2 justify-center items-center text-3xl font-black"
    >
      <Library />
      <h1>Logos</h1>
    </a>
  </header>
)

const AuthLayout: FC<PropsWithChildren> = ({ children }) => {
  return (
    <>
      <Header />
      {children}
    </>
  )
}

export default AuthLayout
