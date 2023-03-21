import type { NextRequest } from "next/server"
import { NextResponse } from "next/server"

export async function middleware(req: NextRequest) {
  return req.cookies.has("id")
    ? NextResponse.next()
    : NextResponse.redirect(
        new URL(`/auth?redirect=${req.nextUrl.pathname}`, req.nextUrl.origin)
      )
}

export const config = {
  matcher: ["/((?!auth|_next/static|_next/image|favicon.ico|robots.txt).+)"],
}
