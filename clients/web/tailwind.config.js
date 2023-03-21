const { fontFamily } = require("tailwindcss/defaultTheme")

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      fontFamily: {
        sans: ["var(--font-sans)", ...fontFamily.sans],
        serif: ["var(--font-serif)", ...fontFamily.serif],
      },
    },
  },
  experimental: ["optimizeUniversalDefaults"],
  future: ["hoverOnlyWhenSupported", "respectDefaultRingColorOpacity"],
  plugins: [require("@tailwindcss/typography"), require("tailwindcss-animate")],
}
