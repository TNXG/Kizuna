/** @type {import('tailwindcss').Config} */
import daisyui from "daisyui"

const twConfig = {
  content: [
    "**/**/*.{js,vue,ts,html}",
    "**/*{js,vue,ts,html}",
  ],
  theme: {
    extend: {},
  },
  plugins: [
    daisyui,
  ],
}

export default twConfig