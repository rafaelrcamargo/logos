import { cookies } from "next/headers"
import Image from "next/image"

type Candidade = {
  id: string
  image: string
  name: string
}

const logError = (err: any) => {
  console.error("Error: ", JSON.stringify(err))
  return undefined
}

const getRecommendation = async (
  category: string,
  session: string | undefined
) => {
  if (!session) return undefined

  const res = await fetch(
    `http://localhost/api/v1/user/recommendation/${category}`,
    {
      headers: {
        cookie: `id=${session}`,
      },
    }
  )

  if (!res.ok) return logError(res.status)

  try {
    const json = await res.json()
    if (json?.error) return logError(json.error)

    try {
      return json as Candidade[]
    } catch (e) {
      return logError(e)
    }
  } catch (e) {
    return logError(e)
  }
}

export const FW = async () => {
  const session = cookies().get("id")?.value
  const candidates = await getRecommendation("follow-worthy", session)

  return (
    <div className="flex flex-col items-center">
      <h1 className="mt-4 text-xl font-bold">Follow-worthy accounts:</h1>
      <div className="flex gap-4">
        {candidates &&
          candidates.map(candidate => (
            <div
              key={candidate.id}
              className="flex flex-col items-center duration-150 hover:scale-105"
            >
              <Image
                src={candidate.image}
                alt={candidate.name}
                width={86}
                height={86}
                className="m-0 rounded-full shadow-sm"
              />
              <p className="text-sm">{candidate.name}</p>
            </div>
          ))}
      </div>
    </div>
  )
}
