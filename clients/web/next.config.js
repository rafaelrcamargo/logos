/** @type {import('next').NextConfig} */
const nextConfig = {
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
