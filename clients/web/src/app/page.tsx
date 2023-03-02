import { Suspense } from "react"
const Hello = dynamic(() => import("components/hello"), { ssr: false })

import Loading from "app/loading"
import dynamic from "next/dynamic"

export default function Home() {
  return (
    <main className={"center flex-col gap-8 font-sans"}>
      <Suspense fallback={<Loading />}>
        <Hello />
      </Suspense>
    </main>
  )
}
