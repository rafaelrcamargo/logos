"use client"

import { FC } from "react"

interface Props {
  error: Error
  reset: () => void
}

export const Error: FC<Props> = ({ error, reset }) => {
  return (
    <html
      lang="en"
      className={
        "bg-neutral-100 text-neutral-900 dark:bg-neutral-900 dark:text-neutral-100"
      }
    >
      <head></head>
      <body className="center overflow-x-hidden font-sans antialiased gap-4">
        <div className="flex gap-2">
          <h2 className="text-2xl">Something went wrong!</h2>
          <p className="text-md">{JSON.stringify(error.message)}</p>
        </div>
        <button
          className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
          onClick={() => reset()}
        >
          Try again
        </button>
      </body>
    </html>
  )
}
