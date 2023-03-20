/** @type {import('next').NextConfig} */
const nextConfig = {
  images: {
    domains: ["avatars.githubusercontent.com"],
  },
  experimental: {
    // mdxRs: true,
    appDir: true,
    typedRoutes: true,
    fontLoaders: [
      { loader: "@next/font/google", options: { subsets: ["latin"] } },
    ],
  },
}

module.exports = nextConfig
