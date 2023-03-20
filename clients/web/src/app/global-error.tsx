"use client"

import { FC } from "react"

interface Props {
  error: Error
  reset: () => void
}

export const Error: FC<Props> = ({ error, reset }) => {
  return (
    <html>
      <head></head>
      <body>
        <h2>Something went wrong!</h2>
        <p>{JSON.stringify(error)}</p>
        <button onClick={() => reset()}>Try again</button>
      </body>
    </html>
  )
}
