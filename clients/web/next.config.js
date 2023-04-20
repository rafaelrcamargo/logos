/** @type {import('next').NextConfig} */
const nextConfig = {
  images: {
    domains: ["i.scdn.co", "picsum.photos", "avatars.githubusercontent.com"],
  },
  experimental: {
    appDir: true,
    // mdxRs: true,
    typedRoutes: true,
  },
}

module.exports = nextConfig
