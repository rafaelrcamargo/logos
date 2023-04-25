import { ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export const warn = (err: unknown) => {
  console.warn("Error: ", JSON.stringify(err))
  return undefined
}

export const get = async <T>(
  path: string,
  session?: string,
  opts?: RequestInit
) => {
  const resp = await fetch(`http://localhost/api/v1${path}`, {
    headers: {
      ...(session && { Cookie: `id=${session}` }),
      ...opts?.headers,
    },
    ...opts,
  })

  if (!resp.ok) return warn(resp.status)

  try {
    const json = (await resp.json()) as JSONResponse<T>
    return json?.error ? warn(json.error) : json
  } catch (error) {
    return warn(error)
  }
}
