"use client"

export default () => (
  <>
    {document.cookie.includes("id") ? (
      <h1 className="font-black text-6xl dark:text-zinc-100 text-zinc-900">
        Welcome to Logos!
      </h1>
    ) : (
      <>
        <h1 className="font-black text-6xl dark:text-zinc-100 text-zinc-900">
          Sign in:
        </h1>
        <div className="flex flex-col gap-4">
          <a
            href={"http://127.0.0.1:8081/api/v1/oauth/github/create"}
            className="text-xl font-bold text-blue-500 underline"
          >
            Github
          </a>
        </div>
      </>
    )}
  </>
)
