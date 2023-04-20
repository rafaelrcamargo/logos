import type { Candidade } from "types/api"

import { cookies } from "next/headers"
import Image from "next/image"

import { get } from "lib/utils"

export const FW = async () => {
  const session = cookies().get("id")?.value
  const candidates = session
    ? await get<Candidade[]>("/user/recommendation/follow-worthy", session)
    : undefined

  return (
    <>
      {candidates ? (
        <div className="flex flex-col items-center">
          <h1 className="mt-4 text-xl font-bold">Follow-worthy accounts:</h1>
          <div className="flex gap-4">
            {candidates.map(({ id, name, image }) => (
              <div
                key={id}
                className="flex flex-col items-center duration-150 hover:scale-105"
              >
                <Image
                  src={image}
                  alt={name}
                  width={86}
                  height={86}
                  className="m-0 rounded-full shadow-sm"
                />
                <p className="text-sm">{name}</p>
              </div>
            ))}
          </div>
        </div>
      ) : (
        <div />
      )}
    </>
  )
}
